模範解答は、Book の第 21 章のスレッドプールにならっています。プールは 1 つの `mpsc` チャネルの
送信側を持ち、各ワーカーは「次のジョブを受け取って実行する」をくり返します。`Receiver` は複製できず、
`Sync` でもないので、ワーカーは `Arc<Mutex<Receiver<Job>>>` として共有します。ミューテックスが
あるので、どのジョブもちょうど 1 つのワーカーだけが受け取ります。

クロージャはそれぞれ別の型を持つので、キューにはジョブをトレイトオブジェクトとして入れます。
`type Job = Box<dyn FnOnce() + Send + 'static>` です。ジョブは 1 回だけ実行される（そしてキャプチャした値を
消費してよい）ので `FnOnce`、ワーカースレッドに移動するので `Send`、`execute` から戻ったずっと後に
実行されることもあるので `'static` です。`Box<dyn FnOnce()>` は `job()` でそのまま呼び出せます。

**正しい順番で止める。** `recv` はキューに残っているジョブを渡し続け、キューが空で、**かつ**すべての
`Sender` がなくなったときに初めて失敗します。そこで `stop` は、まずプールの `Sender` を破棄します。
これでワーカーはキューを片付けてからループを抜けます。その後で初めてワーカーを `join` します。
先に `join` すると、ワーカーは `recv` で永遠に待ち、`join` はそのワーカーを待つので、Miri は
デッドロックを報告します。送信側を `Option<Sender<Job>>` にしておくと、`stop` は `&mut self` 経由でも
`take()` で破棄できます。

**`self` からのムーブ。** `JoinHandle::join` はハンドルを値で受け取るので、
`for w in &mut self.workers { w.join() }` はコンパイルが通りません（E0507）。
`self.workers.drain(..)` なら、ハンドルを取り出して空の `Vec` を残せます。`shutdown(self)` にも
似た問題があります。`ThreadPool` は `Drop` を実装しているので、フィールドをムーブして取り出すことも、
分解することもできません（E0509）。値全体に対してあとで `drop` が実行されるからです。また、`drop` は
`&mut self` しか受け取らないので、`shutdown` を呼べません。解決策は、両方から使うヘルパー
`stop(&mut self)` を 1 つ用意することです。`stop` はプールを止まった空の状態にします。`shutdown` から
戻るときに `self` が破棄されますが、2 回目の `stop` は送信側もワーカーも見つけないので、何もしません。
（`shutdown` の `self` を `mut self` にしても、シグネチャは変わりません。）

**ジョブを実行する前にロックを手放す。** `let message = receiver.lock().unwrap().recv();` では、
`MutexGuard` は一時値で、`let` 文の終わり、つまりジョブを実行する前に破棄されます。短く書いた
`while let Ok(job) = receiver.lock().unwrap().recv() { job(); }` はそうなりません。`while let`、
`if let`、`match` の検査対象（scrutinee）の一時値はブロックの終わりまで生きるので、ワーカーは
ジョブを実行している間もロックを持ち続けます。ほかのワーカーはジョブを受け取ることさえできず、
プールは一度に 1 つのジョブしか実行しません。非公開ケースのようにジョブどうしが待ち合わせると、
デッドロックになります。（エディション 2024 では、`if let` の一時値は `else` ブロックの前に
破棄されますが、パターンにマッチしたときに実行されるブロックの中ではまだ生きています。）

**プールの共有。** ハーネスは `&ThreadPool` を複数のスレッドで共有するので、`ThreadPool` は `Sync` で
なければなりません。`Sender` は（Rust 1.72 から）`Sync` なので、模範解答ではミューテックスで囲む
必要はありません。

### 別解

`drain` の代わりに `std::mem::take` を使う方法です。`let ... else` のガードも文の終わりで破棄されます。

```rust
use std::mem;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            let receiver = Arc::clone(&receiver);
            workers.push(thread::spawn(move || {
                loop {
                    // The guard is dropped at the end of this `let ... else` statement.
                    let Ok(job) = receiver.lock().unwrap().recv() else {
                        break;
                    };
                    job();
                }
            }));
        }
        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }

    pub fn shutdown(mut self) {
        self.stop();
    }

    fn stop(&mut self) {
        drop(self.sender.take());
        for worker in mem::take(&mut self.workers) {
            worker.join().unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.stop();
    }
}
```

`while let` のまま、`Option<Job>` を返す関数の中でロックする方法もあります。関数から戻るときに
ガードが破棄されるので、ジョブの実行中は何もロックしていません。

```rust
fn run_worker(receiver: &Mutex<Receiver<Job>>) {
    while let Some(job) = next_job(receiver) {
        job();
    }
}

/// Waits for the next job. The guard is dropped when this function returns.
fn next_job(receiver: &Mutex<Receiver<Job>>) -> Option<Job> {
    receiver.lock().unwrap().recv().ok()
}
```

すでに閉じたチャネルの `Sender`（その `Receiver` はすぐに破棄されます）と入れ替えれば、送信側を
`Option` で包まずに済みます。フィールドはただの `Sender<Job>` になり、`execute` は
`self.sender.send(...)` を直接呼びます。

```rust
fn stop(&mut self) {
    let (closed, _) = mpsc::channel();
    drop(mem::replace(&mut self.sender, closed));
    for worker in self.workers.drain(..) {
        worker.join().unwrap();
    }
}
```

チャネルを使わない方法です。「閉じたか」のフラグを持つ `Mutex<VecDeque<Job>>` と、キューが空で
開いている間ワーカーが眠るための `Condvar` を使います。閉じるときはすべてのワーカーを起こす必要が
あるので `notify_all` です。

```rust
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Queue {
    jobs: VecDeque<Job>,
    closed: bool,
}

struct Shared {
    queue: Mutex<Queue>,
    /// Notified when a job is added or the queue is closed.
    changed: Condvar,
}

impl Shared {
    /// Waits for the next job. Returns `None` once the queue is closed and empty.
    fn next_job(&self) -> Option<Job> {
        let queue = self.queue.lock().unwrap();
        let mut queue = self
            .changed
            .wait_while(queue, |q| q.jobs.is_empty() && !q.closed)
            .unwrap();
        queue.jobs.pop_front()
    }
}

pub struct ThreadPool {
    shared: Arc<Shared>,
    workers: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let shared = Arc::new(Shared {
            queue: Mutex::new(Queue { jobs: VecDeque::new(), closed: false }),
            changed: Condvar::new(),
        });
        let workers = (0..size)
            .map(|_| {
                let shared = Arc::clone(&shared);
                thread::spawn(move || {
                    // `next_job` releases the lock before it returns.
                    while let Some(job) = shared.next_job() {
                        job();
                    }
                })
            })
            .collect();
        ThreadPool { shared, workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.shared.queue.lock().unwrap().jobs.push_back(Box::new(f));
        self.shared.changed.notify_one();
    }

    pub fn shutdown(mut self) {
        self.stop();
    }

    fn stop(&mut self) {
        self.shared.queue.lock().unwrap().closed = true;
        self.shared.changed.notify_all();
        for worker in self.workers.drain(..) {
            worker.join().unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.stop();
    }
}
```
