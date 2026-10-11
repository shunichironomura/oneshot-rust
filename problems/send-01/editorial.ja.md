それぞれの関数には、本体が必要とする境界を過不足なく書きます。2 つの本体は `f` を
逆の方法で使うので、テンプレートの `FnMut` はどちらにとっても間違いで、その方向も逆です。

**`par_map`** は、`let f = &f;` で同じクロージャをすべてのスコープ付きスレッドに渡します。
スレッドの間を移るのは共有参照 `&F` で、`F` そのものではありません。`&F` が `Send` になるのは
`F` が `Sync` のときに限られます（これが `Sync` の定義です）。したがって必要な境界は `F: Send`
ではなく `F: Sync` です。複数のスレッドが `&F` を通して同時に `f` を呼ぶので、`f` は `Fn` で
なければなりません。`FnMut` を呼ぶには `&mut F` が必要です（E0596）。各要素はそれぞれのスレッドへ
ムーブされ（`T: Send`）、各結果は `join` で呼び出し元へムーブされます（`U: Send`）。`thread::scope`
は戻る前にすべてのスレッドを `join` するので、`'static` はどこにも要りません。ハーネスはこれを
前提にしていて、クロージャは `calls` と `prefix` を借用し、あるケースでは要素と結果が `text` を
借用しています。

**`spawn_job`** が使う `thread::spawn` のスレッドは呼び出し元より長生きするかもしれないので、
クロージャ `move || f(input)` とその結果は `Send + 'static` でなければなりません。このクロージャは
`f` と `input` を所有するので `F: Send + 'static` と `T: Send + 'static` が、結果は `JoinHandle<U>`
を通って戻ってくるので `U: Send + 'static` が必要です。`f` は 1 回しか呼ばないので `FnOnce` で
十分で、ハーネスもちょうどそれを求めています。`move |n: usize| name + ...` はキャプチャした
`String` を手放すので、`FnMut` でも `Fn` でもありません（E0507）。

「念のため」に境界を足すと、ハーネスが通らなくなります。`par_map` に `F: Send + Sync` と書くと、
`MutexGuard` を所有するクロージャを渡せません。ロックはロックしたスレッドで解放しなければ
ならないので、ガードは `Sync` ですが `Send` ではないからです。このクロージャは呼び出し元の
スレッドから出ることはなく、`par_map` の最後にそのスレッドで破棄されます。逆に `Receiver` と
`Cell` は `Send` ですが `Sync` ではありません。そのため `spawn_job` に `F: Sync` と書くと、
`Receiver` を所有するクロージャを渡せなくなります。同じように `T: Sync` は `Receiver` の要素や
入力を、`U: Sync` は `Cell` を返すクロージャや、受け取った `Receiver` を返すジョブを拒否します。
`par_map` に `'static` を付けると、ローカル変数を借用するクロージャや要素を渡せなくなります。

### 別解

引数位置の `impl Trait` でも書けます。

```rust
use std::thread::{self, JoinHandle};

pub fn par_map<T: Send, U: Send>(items: Vec<T>, f: impl Fn(T) -> U + Sync) -> Vec<U> {
    thread::scope(|s| {
        let f = &f;
        let handles: Vec<_> = items.into_iter().map(|x| s.spawn(move || f(x))).collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    })
}

pub fn spawn_job<T, U>(input: T, f: impl FnOnce(T) -> U + Send + 'static) -> JoinHandle<U>
where
    T: Send + 'static,
    U: Send + 'static,
{
    thread::spawn(move || f(input))
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
