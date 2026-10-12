次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

`Profile` はユーザー名を持ち、ニックネームとメールアドレスは持つことも持たないこともあります。

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub username: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
}

impl Profile {
    pub fn display_name(&self) -> &str;
    pub fn email_domain(&self) -> Option<&str>;
    pub fn take_nickname(&mut self) -> Option<String>;
}
```

- `display_name` は、ニックネームがあればそれを、なければユーザー名を返します。空のニックネーム `Some("")` も
  ニックネームとして扱うので、そのときの結果は `""` です。
- `email_domain` は、メールアドレスの**最初の** `@` より後ろの部分を返します。この部分は空のこともあり、
  `"abc@"` なら `Some("")` です。メールアドレスがないときや、`@` を含まないときは `None` を返します。
- `take_nickname` はニックネームを取り除いて返します。呼び出した後、プロフィールにニックネームはありません
  （`nickname` は `None` です）。もともとニックネームがなければ `None` を返します。
- どのメソッドも、ここに書いたこと以外は変更しません。`username` と `email` はそのままです。

構造体とシグネチャは変えないでください。

### 例

- メールアドレスが `"a@b.c"` なら、`email_domain()` は `Some("b.c")` を返します。
- ユーザー名が `"alice"` でニックネームが `"ace"` のとき、`display_name()` は `"ace"` です。続けて
  `take_nickname()` を呼ぶと `Some("ace")` が返り、`display_name()` はユーザー名の `"alice"` に戻ります。
