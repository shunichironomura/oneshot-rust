待っているスレッドが見るもの、つまり要素と「閉じたか」のフラグは、すべて 1 つの `Mutex<State<T>>` に入れます。
待つ仕組みは `Condvar` が担います。`wait` はロックを手放すのと眠るのを一度に行い、戻る前にロックを取り直します。
`wait_while(guard, condition)` はこれをループで包んだもので、`condition` が真の間は眠り続け、ガードを返します。
模範解答は待つ側の種類ごとに Condvar を 1 つずつ使います。`pop` は `not_empty` で、`push` は `not_full` で待ちます。

起こされるたびに、条件を確かめ直す必要があります。起こされたというのは、何かが変わったかもしれないというだけです。
起こされたスレッドがロックを取り直す前に、ほかのスレッドが要素（や空き）を取っていくことがあります。
また `Condvar` は、理由もなくスレッドを起こすことさえあります（spurious wakeup）。
`if ... { guard = cv.wait(guard).unwrap(); }` と書くと、そうした `pop` は開いている待ち行列から `None` を返し、
そうした `push` はいっぱいの待ち行列に要素を追加してしまいます。非公開ケースでは、ほかのスレッドが先に
要素や空きを取っていきます。`wait_while` や、`wait` を囲む `while` ループなら、これらをすべて扱えます。

そして、変更のたびに、それを待っているスレッドを起こす必要があります。`push` は待っている `pop` を 1 つ起こし
（`not_empty.notify_one()`）、`pop` は待っている `push` を 1 つ起こします。閉じると、待っている**すべての**
呼び出しの結果が変わるので、`close` は両方の Condvar に `notify_all` が必要です。`notify_one` だったり、
まったく通知しなかったりすると、ほかの待っているスレッドは永遠に眠ったままになり、Miri は
「the evaluated program deadlocked」と報告します。`push` と `pop` の両方で 1 つの Condvar を使う場合は、
`notify_one` が違う種類のスレッドを起こすことがあります（`push` しか進めないのに `pop` を起こすなど）。
起こされたスレッドはまた眠り、通知は失われます。非公開ケースでは、これも起こります。
その場合はどこでも `notify_all` を使ってください（別解を参照）。

ほかにも注意点があります。

- `wait` と `wait_while` はガードを値で受け取って返します。眠っている間はロックを手放すからです。
  `state = cv.wait(state).unwrap()` のように、結果を代入してください。
- 「閉じたか」のフラグは Mutex の中に置く必要があります。ロックを取らずに設定する `AtomicBool` にすると、
  `pop` が確かめてから眠るまでの間に `close` がフラグを立てて通知してしまい、その `pop` が永遠に眠ることがあります。
  ロックの中なら、`close` から見て、確かめることと眠ることは一度に起きます。
- `Mutex` は再入可能ではありません。ガードを持ったまま `push` の中から `self.len()` を呼ぶとデッドロックします。
- `push` は要素を `Err(item)` で返します。要素が待ち行列に移動するのは成功したときだけなので、
  この時点ではまだ `item` を所有しています。
- `Mutex<State<T>>` は `T: Send` なら `Sync` で、`Condvar` は常に `Sync` です。そのため、何もしなくても
  `BoundedQueue<T>` は `Sync` になり、ハーネスは `&BoundedQueue<T>` をスレッド間で共有できます。
- 模範解答はガードを手放してから通知します。起こされたスレッドがすぐにロック待ちにならずに済むからです。
  ガードを持ったまま通知しても正しく動きます。

### 別解

Condvar を 1 つにして、変更のたびに `notify_all` し、`wait_while` の代わりに `while` ループを明示的に書く方法です。
簡単になりますが、変更のたびに待っているスレッドをすべて起こし、その多くはまた眠ることになります。

```rust
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct BoundedQueue<T> {
    capacity: usize,
    state: Mutex<State<T>>,
    /// Notified after every change: push, pop or close.
    changed: Condvar,
}

struct State<T> {
    items: VecDeque<T>,
    closed: bool,
}

impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize) -> Self {
        BoundedQueue {
            capacity,
            state: Mutex::new(State { items: VecDeque::new(), closed: false }),
            changed: Condvar::new(),
        }
    }

    pub fn push(&self, item: T) -> Result<(), T> {
        let mut state = self.state.lock().unwrap();
        while !state.closed && state.items.len() == self.capacity {
            state = self.changed.wait(state).unwrap();
        }
        if state.closed {
            return Err(item);
        }
        state.items.push_back(item);
        self.changed.notify_all();
        Ok(())
    }

    pub fn pop(&self) -> Option<T> {
        let mut state = self.state.lock().unwrap();
        while !state.closed && state.items.is_empty() {
            state = self.changed.wait(state).unwrap();
        }
        let item = state.items.pop_front();
        self.changed.notify_all();
        item
    }

    pub fn close(&self) {
        self.state.lock().unwrap().closed = true;
        self.changed.notify_all();
    }

    pub fn len(&self) -> usize {
        self.state.lock().unwrap().items.len()
    }
}
```
