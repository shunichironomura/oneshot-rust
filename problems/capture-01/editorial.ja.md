Rust 2021 以降、クロージャは変数全体ではなく、実際に使う**場所**（フィールドなど）をキャプチャします。
このクロージャが使っているのは `base.0` だけなので、`move` クロージャであっても、キャプチャするのは
そのフィールド、つまり `*mut u64` だけです。クロージャが `Send` になるのはキャプチャしたものがすべて
`Send` のときだけで、生ポインタは `Send` ではないため、`thread::spawn` はこのクロージャを受け付けません。
`unsafe impl Send` を付けたラッパーの `SendPtr` は、そもそもキャプチャされていないのです。
Rust 2018 では変数 `base` 全体がキャプチャされたので、同じコードがコンパイルできていました。
エディションガイドでも、まさにこの形が例として挙げられています。

直し方は、クロージャの中で変数 `base` そのものを使うことです。模範解答では `self` を受け取るメソッドを
追加しました。`base.get()` は `base` 全体を使うので、クロージャは `Send` である `SendPtr` 全体を
キャプチャします。1 つのクロージャの中だけで直す方法と違い、このラッパーを使うどのクロージャでも
正しく働きます。

正しそうに見えても、フィールドしかキャプチャしない書き方があります。

- クロージャの中で `let SendPtr(p) = base;` と分解する：分解で使われるのは `base.0` だけです。
- クロージャの中で `SendPtr(base.0)` と包み直す：キャプチャしたフィールドから新しいラッパーを
  作っているだけです。

ベースのポインタは、テンプレートのとおり、スレッドを生成する前に**一度だけ**取ってください。
ループのたびに `out.as_mut_ptr()` を呼ぶと、先に生成したスレッドが前のポインタで書き込んでいる最中に
`out` 全体を可変で借用し直すことになり、Miri が未定義動作（データ競合）を報告します。
戻る前にすべてのスレッドを `join` することも大切です。ポインタが有効なのは、`out` を借用している間
だけだからです。

### 別解

クロージャの先頭で変数全体を使う方法です。`let base = base;` で `SendPtr` 全体をクロージャにコピーします。
`let _ = &base;` でも同じ効果があり、`cargo fix --edition` で Rust 2021 に移行するときに挿入されるのは
この行です。

```rust
handles.push(thread::spawn(move || {
    let base = base;
    for i in (w..len).step_by(workers) {
        // SAFETY: `i < len`, and no other thread writes to index `i`.
        unsafe { *base.0.add(i) = (i * i) as u64 };
    }
}));
```

`unsafe` を使わずに書くこともできます。`thread::scope` ならスレッドから `out` を借用でき、`chunks_mut` で
各スレッドに連続した部分を 1 つずつ渡せます。塊の大きさは 1 以上にする必要があります。また、塊の中での
添字は `out` での添字に直してください。

```rust
use std::thread;

pub fn fill_squares(out: &mut [u64], workers: usize) {
    // At least 1: `chunks_mut(0)` panics when `out` is empty.
    let chunk_size = out.len().div_ceil(workers).max(1);
    thread::scope(|s| {
        for (c, chunk) in out.chunks_mut(chunk_size).enumerate() {
            s.spawn(move || {
                let start = c * chunk_size;
                for (j, x) in chunk.iter_mut().enumerate() {
                    let i = start + j;
                    *x = (i * i) as u64;
                }
            });
        }
    });
}
```

互い違いの割り振りも `unsafe` なしで書けます。各要素の `&mut u64` を担当のワーカーに配ってから、
ワーカーごとの分をスコープ付きスレッドにムーブします。

```rust
use std::thread;

pub fn fill_squares(out: &mut [u64], workers: usize) {
    let mut shares: Vec<Vec<(usize, &mut u64)>> = (0..workers).map(|_| Vec::new()).collect();
    for (i, x) in out.iter_mut().enumerate() {
        shares[i % workers].push((i, x));
    }
    thread::scope(|s| {
        for share in shares {
            s.spawn(move || {
                for (i, x) in share {
                    *x = (i * i) as u64;
                }
            });
        }
    });
}
```

スレッドでは 2 乗を計算して返すだけにすれば、`thread::spawn` のままでも書けます。
`join` した後に、呼び出し側のスレッドが `out` に書き込みます。

```rust
use std::thread;

pub fn fill_squares(out: &mut [u64], workers: usize) {
    let len = out.len();
    let handles: Vec<_> = (0..workers)
        .map(|w| {
            thread::spawn(move || {
                (w..len).step_by(workers).map(|i| (i * i) as u64).collect::<Vec<_>>()
            })
        })
        .collect();
    for (w, handle) in handles.into_iter().enumerate() {
        let squares = handle.join().unwrap();
        for (x, square) in out.iter_mut().skip(w).step_by(workers).zip(squares) {
            *x = square;
        }
    }
}
```

アドレスを `usize` にして渡す方法（`out.as_mut_ptr() as usize` とし、スレッドの中で `addr as *mut u64` に
戻す）も、`usize` は `Send` なのでコンパイルが通り、テストにも合格します。ただし、整数をポインタに戻すと
そのポインタがどこから来たのかがわからなくなり、Miri などのツールも正確には検査できなくなります。
意図をはっきり表せるのはラッパー型のほうです。
