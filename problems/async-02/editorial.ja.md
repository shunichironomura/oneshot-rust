`async || { calls += 1; ... }` のような async クロージャは、`&mut calls` をクロージャの中に持ちます。
そして、返す Future はどれも、その状態を**クロージャから**借用します。つまり Future が有効なのは、
それを作った呼び出しが `f` を借用している間だけです。`F: FnMut() -> Fut` ではこれを表せません。
`Fut` はすべての呼び出しで同じ 1 つの型なので、1 回の呼び出しの間だけ存在する `f` の借用を
型に含められないからです。コンパイラが「async closure does not implement `FnMut` because it
captures state from its environment」と言うのはこのためです。`async move ||` にしても解決しません。
状態はクロージャが所有するようになりますが、Future はやはりそれをクロージャから借用します。

Rust 1.85 で、async クロージャと一緒に `AsyncFn`、`AsyncFnMut`、`AsyncFnOnce` の 3 つのトレイトが
追加されました（プレリュードに入っています）。`F: AsyncFnMut() -> Result<T, E>` は「`f` を呼ぶと、
返された Future が完了するまで `f` を可変で借用する」という意味で、本体の使い方にちょうど合います。
各 `f().await` は、次の呼び出しの前に完了するからです。ほかの 2 つは合いません。

- `AsyncFn` は `f` を不変でしか借用しないので、キャプチャしたものを変更するクロージャ
  （`calls += 1`、`log.push(...)`、`fetch_any` の中で `urls` を進めるクロージャ）を受け付けません。
- `AsyncFnOnce` は 1 回しか呼べませんが、本体はループの中で `f` を呼びます（E0382）。

`AsyncFnMut` を呼ぶには `&mut f` が必要なので、引数は `mut f` と `mut fetch` のままにします。
Future の型にはもう名前を付けないので、型引数から `Fut` も取り除きます。引数から `Fut` を決める
手がかりがなくなるので、残しておくと、呼び出すたびに「type annotations needed」（E0282）になります。

`fetch_any` にはもう 1 つ必要なことがあります。本体は、内側のクロージャの中にしかない `String` への
参照 `&url` を渡しており、`fetch` の Future はその参照を持ち続けます。`async |url: &str|` クロージャや
`async fn probe(url: &str)` の Future は、必ず引数を保持するからです。そのため Future の型は引数の
ライフタイムに依存し、すべてのライフタイムに対して 1 つの `Fut` を使う `F: FnMut(&str) -> Fut` は
`probe` で失敗します（E0271）。`F: AsyncFnMut(&str) -> Result<T, E>` は
`F: for<'a> AsyncFnMut(&'a str) -> Result<T, E>` の略記で、Future は引数を借用してかまいません
（借用できないのは出力の `Result<T, E>` だけです）。`AsyncFnMut(&'a str)` のように呼び出し側が
決めるライフタイム引数を使う方法はうまくいきません。`url` は `fetch_any` の中で破棄されるからです（E0597）。

`AsyncFnMut` は、Future を返す普通のクロージャや関数、`probe` のような `async fn` にも実装されて
いるので、古いシグネチャ向けの呼び出しもそのまま動きます（`hidden-4` のクロージャのように、
キャプチャしたものへの共有参照しか必要としない `move` でない async クロージャは、古い境界も
満たしていました。その Future はクロージャを借用せず、それらの参照をコピーして持つからです）。

### 別解

引数位置の `impl Trait` を使うと、クロージャの型引数は要りません。

```rust
pub async fn retry<T, E>(times: usize, mut f: impl AsyncFnMut() -> Result<T, E>) -> Result<T, E>

pub async fn fetch_any<T, E>(
    mirrors: &[&str],
    path: &str,
    mut fetch: impl AsyncFnMut(&str) -> Result<T, E>,
) -> Result<T, E>
```

`fetch_any` の高階トレイト境界は明示的にも書けます。

```rust
pub async fn fetch_any<T, E, F>(mirrors: &[&str], path: &str, mut fetch: F) -> Result<T, E>
where
    F: for<'a> AsyncFnMut(&'a str) -> Result<T, E>,
```
