`async` ブロックはクロージャと同じように変数をキャプチャし、参照で足りるなら参照でキャプチャします。
フォーマットには `&name` しか必要ないので、返される Future はローカル変数 `name` を借用しますが、
`name` は `greet` から戻るときに破棄されます（E0373「async block may outlive the current function」）。
`async move` にすれば、`name` は Future の中にムーブされます。

- `async fn` ではこの問題は起きません。引数は必ず Future の中にムーブされるからです。そのため
  `pub async fn greet(name: String) -> String` と書けばコンパイルは通りますが、この問題では
  シグネチャを変えられません。
- ブロックの中で `name.clone()` と書いても解決しません。クローンが実行されるのは Future が
  ポーリングされたときなので、ブロックは結局 `name` を借用します。

### 別解

先に文字列を作り、すでに完了している Future を返すこともできます。結果は同じですが、
フォーマットは最初のポーリング時ではなく、`greet` を呼んだときに行われます。

```rust
pub fn greet(name: String) -> impl Future<Output = String> {
    std::future::ready(format!("Hello, {name}!"))
}
```
