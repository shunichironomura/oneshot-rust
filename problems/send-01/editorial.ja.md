本体が必要とする境界を過不足なく書きます。

本体は、`let f = &f;` で同じクロージャをすべてのスコープ付きスレッドに渡します。
スレッドの間を移るのは共有参照 `&F` で、`F` そのものではありません。`&F` が `Send` になるのは
`F` が `Sync` のときに限られます（これが `Sync` の定義です）。したがって必要な境界は `F: Send`
ではなく `F: Sync` です。複数のスレッドが `&F` を通して同時に `f` を呼ぶので、`f` は `Fn` で
なければなりません。`FnMut` を呼ぶには `&mut F` が必要です（E0596）。各要素はそれぞれのスレッドへ
ムーブされ（`T: Send`）、各結果は `join` で呼び出し元へムーブされます（`U: Send`）。`thread::scope`
は戻る前にすべてのスレッドを `join` するので、`'static` はどこにも要りません。ハーネスはこれを
前提にしていて、クロージャは `calls` と `prefix` を借用し、あるケースでは要素と結果が `text` を
借用しています。

「念のため」に境界を足すと、ハーネスが通らなくなります。`F: Send + Sync` と書くと、
`MutexGuard` を所有するクロージャを渡せません。ロックはロックしたスレッドで解放しなければ
ならないので、ガードは `Sync` ですが `Send` ではないからです。このクロージャは呼び出し元の
スレッドから出ることはなく、`par_map` の最後にそのスレッドで破棄されます。逆に `Receiver` と
`Cell` は `Send` ですが `Sync` ではありません。そのため `T: Sync` は `Receiver` の要素を、
`U: Sync` は `Cell` を返すクロージャを拒否します。`'static` を付けると、ローカル変数を借用する
クロージャや要素を渡せなくなります。

### 別解

引数位置の `impl Trait` でも書けます。

```rust
use std::thread;

pub fn par_map<T: Send, U: Send>(items: Vec<T>, f: impl Fn(T) -> U + Sync) -> Vec<U> {
    thread::scope(|s| {
        let f = &f;
        let handles: Vec<_> = items.into_iter().map(|x| s.spawn(move || f(x))).collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    })
}
```

`F: Sync` は「`&F` が `Send` である」という意味なので、参照のライフタイムについての高階トレイト
境界を使って、そのとおりに書くこともできます。

```rust
pub fn par_map<T, U, F>(items: Vec<T>, f: F) -> Vec<U>
where
    T: Send,
    U: Send,
    F: Fn(T) -> U,
    for<'a> &'a F: Send,
```
