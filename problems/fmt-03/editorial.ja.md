`#[derive(Debug)]` の表示は、`Formatter` のデバッグ用のビルダーが書いています。それとまったく同じ
表示にするには、同じビルダーを使うのが近道です。`f.debug_struct(name)`（タプルの形なら
`f.debug_tuple(name)`）から始め、フィールドごとに `.field(...)` を呼んで、最後に `.finish()` で
終えます。区切りの記号はビルダーが書き、1 行にするか整形した形にするかは `f.alternate()` で
決まります。整形した形では、フィールドが書いた各行をビルダーが字下げします。入れ子のリストや
マップが正しく表示され、値全体を `Vec` に入れても正しく字下げされるのはこのためです。また、
各フィールドは `f` と同じ設定で書式化されるので、`u8` や `u32` のフィールドは `{:x?}` で
16 進数になり、`#` もあれば `0x` が付きます。

素直に書くとつまずくのは、伏せたパスワードです。フィールドは、その値の `Debug` で表示されます。
`.field("password", &"<redacted>")` の値は `&str` で、その `Debug` はクォートを付けます
（特殊文字もエスケープします）。そのため `password: "<redacted>"` になってしまいます。値は、
`Debug` が文字列をそのまま書くものでなければいけません。`format_args!("<redacted>")` は
`fmt::Arguments` を作り、その `Debug` は文字列をそのまま書きます。`Debug` を手で実装した
小さな型でも構いません（別解を参照）。

ビルダーについて、細かい点がもう 2 つあります。

- `finish_non_exhaustive()` は構造体を `..` で閉じます。1 行なら `, .. }`、整形した形なら `..` だけの
  行になります。
- `field` はビルダーを借用した `&mut DebugTuple` を返します。フィールドを付けたり付けなかったり
  するときは、ビルダー自体を `let mut` の変数に入れておきます。
  `let t = f.debug_tuple("Rgba").field(&self.r);` の後の文で `t` を使うと、コンパイルが通りません
  （E0716）。ビルダーはその `let` 文の終わりで破棄される一時値なのに、`t` がまだそれを借用している
  からです。

`Display` と同じように `write!(f, "Rgb({:?}, {:?}, {:?})", self.r, self.g, self.b)` で `Debug` を
書くと、正しく見えるのは `{:?}` のときだけです。各 `{:?}` は、呼び出し側の指定ではなく、その文字列に
書かれた指定で書式化されます。そのため `{:x?}` でも `Rgb(255, 0, 16)` と表示され、`{:#?}` でも
1 行のままです。呼び出し側のフラグは `f` に入っていて、それが見えるのは、ビルダーのように
フィールドを `f` で書式化するコードだけです。

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

タプルの `Debug` は `(255, 0, 16)` のような名前のないタプルの形を表示するので、先に名前を
書いてから `f` をタプルに渡せば、フラグも含めて同じ表示になります。`f` をそのまま使う
`fmt::Debug::fmt(&tuple, f)` でなければいけません。`write!(f, "{:?}", tuple)` では、また
フラグが失われます。

```rust
impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Color { r, g, b, a } = *self;
        if a == 255 {
            f.write_str("Rgb")?;
            fmt::Debug::fmt(&(r, g, b), f)
        } else {
            f.write_str("Rgba")?;
            fmt::Debug::fmt(&(r, g, b, a), f)
        }
    }
}
```
