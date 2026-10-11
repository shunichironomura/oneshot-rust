4 つのメソッドのどれでも `self` は参照なので、フィールドは借用しているだけです。テンプレートは
`self.nickname` と `self.email` を 3 通りの形で値として使っています。値で束縛するパターン `Some(nick)` を
持つ `match` の対象、`?` のオペランド、`let` の右辺です。どれも `Option<String>` を構造体からムーブしますが、
参照の先からムーブすることはできません（E0507）。`&mut self` でも同じです。構造体は呼び出し側のもので、
次の行で新しい値を入れ直すとしても、フィールドが欠けた状態にしてはいけません。そこで、参照を通して
`Option` を見るか、新しい値を入れて古い値を取り出す操作を一度に行います。

- `display_name`：ムーブの奥にもう 1 つエラーがあります。`nick` はローカル変数で、関数から戻るときに
  破棄されるので、`&nick` は返せません（E0515）。結果は `self.nickname` そのものを指す必要があります。
  `as_deref()` は何もムーブせずに `&Option<String>` を `Option<&str>` に変え、`unwrap_or(&self.username)`
  で代わりの値を与えます（`&String` は `&str` に型強制されます）。
- `email_domain`：`?` はオペランドを値で受け取るので、`self.email?` もムーブです。`&self.email?` と
  書いても解決しません。`?` は `&` より強く結合するので、`&(self.email?)` という意味になるからです。
  コンパイラが提案する `self.email.clone()?` ならムーブのエラーは消えますが、今度は `email` がローカルな
  `String` になり、返すスライスがそれを指してしまいます（やはり E0515）。`self.email.as_deref()?` なら
  `self` から借用した `&str` が得られ、返り値の型の要求どおりになります。
- `take_nickname` と `set_nickname`：`Option::take` は `None` を入れて古い値を返し、`Option::replace` は
  `Some(value)` を入れて古い値を返します。どちらも、フィールドを一度も空にせずに、`&mut` を通して古い値を
  取り出せます。

古いエディションの習慣もここでは罠になります。`match &self.nickname { Some(ref nick) => ... }` は
edition 2024 ではエラーです。対象が参照のときは束縛がすでに参照になるので、そこに `ref` を書くことは
できなくなりました。代わりに `match &self.nickname { Some(nick) => ... }` か
`match self.nickname { Some(ref nick) => ... }` と書きます。

### 別解

参照に対して `match` すれば、`match` だけで書けます。`nick` と `email` は `&String` になります。

```rust
pub fn display_name(&self) -> &str {
    match &self.nickname {
        Some(nick) => nick,
        None => &self.username,
    }
}

pub fn email_domain(&self) -> Option<&str> {
    match &self.email {
        Some(email) => email.split_once('@').map(|(_, domain)| domain),
        None => None,
    }
}
```

`split_once` も `Option` を返すので、`email_domain` では `?` を 2 回使えます。

```rust
pub fn email_domain(&self) -> Option<&str> {
    let (_, domain) = self.email.as_deref()?.split_once('@')?;
    Some(domain)
}
```

`std::mem::replace` を使うと、`Option` に限らずどの型でも同じ入れ替えができます。`std::mem::take` は
既定値を持つ型（`Option` なら `None`）で使えます。

```rust
pub fn take_nickname(&mut self) -> Option<String> {
    std::mem::take(&mut self.nickname)
}

pub fn set_nickname(&mut self, nick: &str) -> Option<String> {
    std::mem::replace(&mut self.nickname, Some(nick.to_string()))
}
```

`let old = self.nickname.clone();` でコピーし、残りはテンプレートのままにしてもコンパイルが通り、
テストにも合格します。ただし、元の `String` を 1 行後に捨てるためだけに、もう 1 つ `String` を確保する
ことになります。`take` と `replace` なら、今ある `String` をそのまま渡せます。
