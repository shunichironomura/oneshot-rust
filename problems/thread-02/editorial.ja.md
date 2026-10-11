`thread::spawn` に渡すクロージャは `Send` でなければならず、クロージャが `Send` になるのは、
キャプチャしたものがすべて `Send` のときだけです。コンパイラはここでの問題を 1 つずつしか報告しないので、
最初の 1 つを直すだけでは足りません。

1. `Rc` は `Send` ではありません。参照カウントを同期せずに更新するので、2 つのスレッドが同時にハンドルを
   複製したり破棄したりすると、カウントが壊れるおそれがあります。`Arc` はカウントをアトミックに更新します。
2. `Arc<T>` が `Send` になるのは `T` が `Send + Sync` のときだけです。ハンドルを持つどのスレッドも、
   共有参照を通じて同じ `T` に触れるからです。`RefCell` と `Cell` は `Sync` ではありません。`&` 越しに中身を
   同期なしで書き換えるからです。そのため `Arc<RefCell<_>>` や `Arc<Cell<_>>` もエラーになります。
   スレッド安全な代わりになるのは、`Mutex`（または `RwLock`）と、`AtomicUsize` などのアトミック型です。
3. `Mutex::into_inner` は中身そのものではなく `LockResult` を返します。ロックを持ったまま
   パニックしたスレッドがあると、ミューテックスは「ポイズン」された（poisoned）状態になるからです。
   そのため `unwrap()` がもう 1 つ必要です。

`words` は読むだけなので、`Arc<Vec<String>>` で十分です。ロックは要りません。
`Arc::try_unwrap` が成功するのは、ほかのハンドルが残っていないときだけです。スレッドに move した複製は
クロージャが終わると破棄されるので、すべて `join` した後は元のハンドルが最後の 1 つになります。
カウンタには `Ordering::Relaxed` で十分です。加算が失われることはなく、`join` すると、
そのスレッドが行ったことはすべて `join` した側のスレッドから見えるようになるからです。

### 別解

1 つの `Mutex` でマップとカウンタをまとめて守ることもできます。ロックを取る回数も、
1 文字ごとではなく 1 単語ごとになります。`Arc::into_inner` は `Arc::try_unwrap` と似ていますが、
ほかのハンドルが残っているときは `None` を返します。

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn letter_counts(words: Vec<String>, workers: usize) -> (HashMap<char, usize>, usize) {
    let words = Arc::new(words);
    let state = Arc::new(Mutex::new((HashMap::new(), 0)));

    let mut handles = Vec::new();
    for w in 0..workers {
        let words = Arc::clone(&words);
        let state = Arc::clone(&state);
        handles.push(thread::spawn(move || {
            for word in words.iter().skip(w).step_by(workers) {
                let mut guard = state.lock().unwrap();
                let (counts, processed) = &mut *guard;
                for c in word.chars() {
                    *counts.entry(c).or_insert(0) += 1;
                }
                *processed += 1;
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    Arc::into_inner(state).unwrap().into_inner().unwrap()
}
```

`Arc` から取り出す代わりに、最後にもう一度ロックして、`std::mem::take` でマップを取り出す方法もあります
（代わりに空のマップが残ります）。ここではカウンタも `Mutex<usize>` にしています。

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn letter_counts(words: Vec<String>, workers: usize) -> (HashMap<char, usize>, usize) {
    let words = Arc::new(words);
    let counts = Arc::new(Mutex::new(HashMap::new()));
    let processed = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();
    for w in 0..workers {
        let words = Arc::clone(&words);
        let counts = Arc::clone(&counts);
        let processed = Arc::clone(&processed);
        handles.push(thread::spawn(move || {
            for word in words.iter().skip(w).step_by(workers) {
                for c in word.chars() {
                    *counts.lock().unwrap().entry(c).or_insert(0) += 1;
                }
                *processed.lock().unwrap() += 1;
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let counts = std::mem::take(&mut *counts.lock().unwrap());
    (counts, *processed.lock().unwrap())
}
```

共有マップを使わない作りなら、各スレッドがローカルな `HashMap` に数えて `JoinHandle` 経由で返し、
呼び出し側のスレッドでまとめることもできます。この場合 `Mutex` はまったく要りません。また、
`thread::spawn` の代わりに `thread::scope` を使えば、クロージャは `Arc` なしで `words`、`Mutex`、
`AtomicUsize` を借用できます。ただし `RefCell` と `Cell` はやはりエラーになります。
`&T` が `Send` になるのは `T` が `Sync` のときだけだからです。
