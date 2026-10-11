`Waker` は、`std::task::Wake` を実装した型 `W` の `Arc<W>` から作れます。ただし
`From<Arc<W>> for Waker` には `W: Wake + Send + Sync + 'static` という条件があります。
ウェイカーは複製されて、どのスレッドからでも起こされるかもしれないからです。この問題のタスクは
`Rc` を持っていて `Send` ではないので、ウェイカーを `Arc<Task>` にして Future を `Mutex` に
入れておく定番の設計はコンパイルできません（`Waker::from` で「`dyn Future<Output = ()>`
cannot be sent between threads safely」）。箱に入れた Future に `+ Send` を付けてもエラーが
`spawn` に移るだけで、キューを `Rc<RefCell<VecDeque<_>>>` にしても同じ理由で失敗します。
そこで、タスクを 2 つに分けます。

- Future は実行器が `Pin<Box<dyn Future<Output = ()>>>` として持ち、タスクの ID で引けるように
  します。ポーリングにはピン留めされた Future が必要です。`Box::pin` はどんな Future でも
  ピン留めでき、`as_mut()` でポーリングのたびにピンを再借用できます。
- ウェイカーが知っているのは、自分のタスクの ID と実行可能キュー
  `Arc<Mutex<VecDeque<usize>>>` だけです。これらは `Send + Sync` なので、
  `Waker::from(Arc::new(TaskWaker { .. }))` がコンパイルできます。

起こすと ID がキューに入り、`run` は ID を取り出して対応する Future をポーリングします。
あとは細かい管理です。

- **重複の除去。** タスクごとに `queued` フラグを持ちます。ウェイカーは `Sync` でなければ
  ならないので `AtomicBool` です。フラグを `false` から `true` に変えたときだけ ID をキューに
  入れるので、タスクは最初に起こされたときの位置を保ちます。`run` はフラグをポーリングの
  **前に**下ろします。後で下ろすと、ポーリング中に起こしたとき（例のタスク A のように）
  ウェイカーが `true` を見て何もキューに入れないので、そのタスクは二度と実行されません。
  起こすたびに ID をキューに入れ、取り出すときにフラグを確かめる方法も同じにはなりません。
  前に起こされたときの古い項目のせいで、タスクが自分より先に実行可能になったタスクを
  追い越すことがあるからです。
- **終了したタスク。** キューには終了したタスクの ID が残っていることがあり（自分を起こしてから
  `Ready` を返した場合）、しまっておいたウェイカーはいつ起こされるかわかりません。終了した
  タスクの場所は `None` にし（これで Future が破棄されます）、`run` はそのような ID を読み飛ばします。
  ID を使い回さないので、古いウェイカーが新しいタスクを起こすことはありません。終了したタスクを
  取り除く `HashMap` で `id = tasks.len()` とすると、そうなってしまいます。
- **ポーリング中はロックを持たない。**
  `while let Some(id) = self.ready.lock().unwrap().pop_front() { ... }` では、`MutexGuard` は
  条件式の一時値で、ループ本体の終わりまで生きています（`match` の対象の式も同じで、`if let` では
  最初のブロックの終わりまで生きます）。ポーリング中に自分を起こすタスクがあると、同じスレッドで
  キューを 2 回ロックすることになり、Miri はデッドロックを報告します。`let ... else` なら一時値は
  文の終わりで破棄されるので、ポーリングの前にガードはなくなっています。

### 別解

`mpsc::Sender` は `Send + Sync` なので（Rust 1.72 から）、ウェイカーにタスク ID の送信側を
持たせ、`run` で受信側から `try_recv` で取り出すこともできます。実行器（と受信側）が破棄された後は
送信が失敗します。そのエラーは `unwrap` せずに無視してください。

```rust
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::task::{Context, Wake, Waker};

struct TaskWaker {
    id: usize,
    queued: AtomicBool,
    sender: Sender<usize>,
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        if !self.queued.swap(true, Ordering::SeqCst) {
            // Fails only when the executor is gone; then there is nothing left to run.
            let _ = self.sender.send(self.id);
        }
    }
}

type Task = (Pin<Box<dyn Future<Output = ()>>>, Arc<TaskWaker>);

pub struct Executor {
    tasks: Vec<Option<Task>>,
    sender: Sender<usize>,
    receiver: Receiver<usize>,
}

impl Executor {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Executor { tasks: Vec::new(), sender, receiver }
    }

    pub fn spawn(&mut self, fut: impl Future<Output = ()> + 'static) {
        let waker = Arc::new(TaskWaker {
            id: self.tasks.len(),
            queued: AtomicBool::new(false),
            sender: self.sender.clone(),
        });
        waker.wake_by_ref();
        self.tasks.push(Some((Box::pin(fut), waker)));
    }

    pub fn run(&mut self) -> usize {
        let mut polls = 0;
        while let Ok(id) = self.receiver.try_recv() {
            let slot = &mut self.tasks[id];
            let Some((future, task_waker)) = slot else {
                continue;
            };
            task_waker.queued.store(false, Ordering::SeqCst);
            let waker = Waker::from(Arc::clone(task_waker));
            polls += 1;
            if future.as_mut().poll(&mut Context::from_waker(&waker)).is_ready() {
                *slot = None;
            }
        }
        polls
    }

    pub fn pending(&self) -> usize {
        self.tasks.iter().filter(|task| task.is_some()).count()
    }
}
```

フラグをキューの隣に置いて同じ `Mutex` で守れば、アトミック型は要りません。この版ではタスクを
`HashMap` に入れ、タスクごとの `Waker` も 1 回だけ作っています。

```rust
use std::collections::{HashMap, VecDeque};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Wake, Waker};

/// Everything the wakers change, behind one lock.
#[derive(Default)]
struct ReadyQueue {
    order: VecDeque<usize>,
    /// `queued[id]`: whether task `id` is in `order`.
    queued: Vec<bool>,
}

impl ReadyQueue {
    /// Registers a new task, which is ready, and returns its id.
    fn add(&mut self) -> usize {
        let id = self.queued.len();
        self.queued.push(true);
        self.order.push_back(id);
        id
    }

    fn wake(&mut self, id: usize) {
        if !self.queued[id] {
            self.queued[id] = true;
            self.order.push_back(id);
        }
    }

    fn pop(&mut self) -> Option<usize> {
        let id = self.order.pop_front()?;
        self.queued[id] = false;
        Some(id)
    }
}

struct TaskWaker {
    id: usize,
    ready: Arc<Mutex<ReadyQueue>>,
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.ready.lock().unwrap().wake(self.id);
    }
}

pub struct Executor {
    tasks: HashMap<usize, (Pin<Box<dyn Future<Output = ()>>>, Waker)>,
    ready: Arc<Mutex<ReadyQueue>>,
}

impl Executor {
    pub fn new() -> Self {
        Executor { tasks: HashMap::new(), ready: Arc::default() }
    }

    pub fn spawn(&mut self, fut: impl Future<Output = ()> + 'static) {
        let id = self.ready.lock().unwrap().add();
        let waker = Waker::from(Arc::new(TaskWaker { id, ready: Arc::clone(&self.ready) }));
        self.tasks.insert(id, (Box::pin(fut), waker));
    }

    pub fn run(&mut self) -> usize {
        let mut polls = 0;
        loop {
            let next = self.ready.lock().unwrap().pop();
            let Some(id) = next else {
                return polls;
            };
            let Some((future, waker)) = self.tasks.get_mut(&id) else {
                continue;
            };
            polls += 1;
            if future.as_mut().poll(&mut Context::from_waker(waker)).is_ready() {
                self.tasks.remove(&id);
            }
        }
    }

    pub fn pending(&self) -> usize {
        self.tasks.len()
    }
}
```
