トークンは `Tokenizer` ではなく**入力文字列**のスライスです。構造体のライフタイム引数を使って
`Option<&'a str>` と書けば、次に `next_token` を呼んだあともトークンは有効なままです。

返り値を省略形（`Option<&str>`）のままにすると、`&mut self` の借用に結び付けられます。するとトークンが
生きている間は `Tokenizer` が可変借用されたままになり、トークンを保持しながら `next_token` を呼び続ける
ハーネスはコンパイルできません。

### 別解

`Iterator` を実装すると、`collect` や `for` ループなどのイテレータの機能をそのまま使えます。

```rust
impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.next_token()
    }
}
```
