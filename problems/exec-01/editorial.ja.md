`Waker` は、`std::task::Wake` を実装した型 `W` の `Arc<W>` から作れます。ただし
`From<Arc<W>> for Waker` には `W: Wake + Send + Sync + 'static` という条件があります。
ウェイカーは複製されて、どのスレッドからでも起こされるかもしれないからです。この問題のタスクは
`Rc` を持っていて `Send` ではないので、ウェイカーを `Arc<Task>` にして Future を `Mutex` に
入れておく定番の設計はコンパイルできません。`Waker::from` のところで、
`dyn Future<Output = ()>` はスレッド間で安全に送れない（cannot be sent between threads safely）
というエラーになります。Box に入れた Future に `+ Send` を付けてもエラーが `spawn` に移るだけで、
キューを `Rc<RefCell<VecDeque<_>>>` にしても同じ理由で失敗します。
そこで、タスクを 2 つに分けます。

- Future はエグゼキュータが `Pin<Box<dyn Future<Output = ()>>>` として持ち、タスクの ID で引けるように
  します。ポーリングにはピン留めされた Future が必要です。`Box::pin` はどんな Future でも
  ピン留めでき、`as_mut()` でポーリングのたびにピンを再借用できます。
- ウェイカーが知っているのは、自分のタスクの ID と実行可能キュー
  `Arc<Mutex<VecDeque<usize>>>` だけです。これらは `Send + Sync` なので、
  `Waker::from(Arc::new(TaskWaker { .. }))` がコンパイルできます。`spawn` は `Waker` を 1 回だけ作り、
  Future と一緒に `Task` に入れておきます。

起こすと ID がキューに入り、`run` は ID を取り出して対応する Future をポーリングします。
ハーネスは、すでに実行可能なタスクを起こさないので、起こされたら常にキューに入れてかまいません
（実際のエグゼキュータはこの場合も扱う必要があります。ふつうはタスクごとに「キューに入っている」
フラグを持ち、2 回起こされたタスクも 1 回だけポーリングします）。あとは細かい管理です。

- **終了したタスク。** キューには終了したタスクの ID が残っていることがあり（自分を起こしてから
  `Ready` を返した場合）、しまっておいたウェイカーはいつ起こされるかわかりません。終了した
  タスクの場所は `None` にし（これで Future が破棄されます）、`run` はそのような ID を `continue` で
  読み飛ばします（`break` や `return` ではいけません。後ろに実行可能なタスクがあるかもしれません）。
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
持たせ、`run` で受信側から `try_recv` で取り出すこともできます。エグゼキュータ（と受信側）が破棄された後は
送信が失敗します。そのエラーは `unwrap` せずに無視してください。

```rust
use std::pin::Pin;
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::task::{Context, Wake, Waker};

/// A spawned task: its future, and the waker to poll it with.
struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
    waker: Waker,
}

struct TaskWaker {
    id: usize,
    sender: Sender<usize>,
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        // Fails only when the executor is gone; then there is nothing left to run.
        let _ = self.sender.send(self.id);
    }
}

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
        let id = self.tasks.len();
        let waker = Waker::from(Arc::new(TaskWaker { id, sender: self.sender.clone() }));
        waker.wake_by_ref();
        self.tasks.push(Some(Task { future: Box::pin(fut), waker }));
    }

    pub fn run(&mut self) -> usize {
        let mut polls = 0;
        while let Ok(id) = self.receiver.try_recv() {
            let slot = &mut self.tasks[id];
            let Some(task) = slot else {
                continue;
            };
            polls += 1;
            if task.future.as_mut().poll(&mut Context::from_waker(&task.waker)).is_ready() {
                *slot = None;
            }
        }
        polls
    }
}
```
