次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

`greet_all` は名前のリストを受け取り、それぞれに `"Hello, <name>!"` という挨拶を作って返します。
あわせて、何人に挨拶したかを返します。

```rust
pub fn greet_all(names: Vec<String>) -> (Vec<String>, usize)
```

シグネチャは変えないでください。

### 例

`greet_all(vec!["Ferris".into()])` は `(vec!["Hello, Ferris!"], 1)` を返します。
