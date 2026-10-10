`Vec<T>` はそれ自体がスタックとして使えます。`push` は末尾に追加し、`pop` は末尾から取り出して `Option<T>` を返し、
`last` は `Option<&T>` を返します。どのメソッドも `Vec` に処理を任せるだけで済みます。

### 別解

引数のない `new()` を持つ型は、`Default` も実装するのが慣例です（Clippy の `new_without_default` が指摘します）。
`#[derive(Default)]` を使うと `Vec<T>` には不要な `T: Default` という境界が付くので、手で実装します。

```rust
impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}
```
