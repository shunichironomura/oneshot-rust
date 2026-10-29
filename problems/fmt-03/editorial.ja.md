`#[derive(Debug)]` の表示は、`Formatter` のデバッグ用のビルダーが書いています。それとまったく同じ
表示にするには、同じビルダーを使うのが近道です。`f.debug_struct(name)` から始め、フィールドごとに
`.field(...)` を呼んで、最後に `.finish()` で終えます。区切りの記号はビルダーが書き、1 行にするか
整形した形にするかは `f.alternate()` で決まります。整形した形では、フィールドが書いた各行を
ビルダーが字下げします。入れ子のリストが正しく表示され、値全体を `Some` に入れても正しく
字下げされるのはこのためです。

素直に書くとつまずくのは、伏せたパスワードです。フィールドは、その値の `Debug` で表示されます。
`.field("password", &"<redacted>")` の値は `&str` で、その `Debug` はクォートを付けます
（特殊文字もエスケープします）。そのため `password: "<redacted>"` になってしまいます。値は、
`Debug` が文字列をそのまま書くものでなければいけません。`format_args!("<redacted>")` は
`fmt::Arguments` を作り、その `Debug` は文字列をそのまま書きます。`Debug` を手で実装した
小さな型でも構いません（別解を参照）。

`Display` と同じように
`write!(f, "Credentials {{ user: {:?}, password: <redacted>, scopes: {:?} }}", self.user, self.scopes)`
で `Debug` を書くと、正しく見えるのは `{:?}` のときだけです。各 `{:?}` は、呼び出し側の指定ではなく、
その文字列に書かれた指定で書式化されます。そのため `{:#?}` でも 1 行のままです。呼び出し側のフラグは
`f` に入っていて、それが見えるのは、ビルダーのようにフィールドを `f` で書式化するコードだけです。

### 別解

`format_args!` の代わりに、`Debug` で文字列をそのまま書く小さな型を使っても構いません。

```rust
struct Redacted;

impl fmt::Debug for Redacted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<redacted>")
    }
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("user", &self.user)
            .field("password", &Redacted)
            .field("scopes", &self.scopes)
            .finish()
    }
}
```
