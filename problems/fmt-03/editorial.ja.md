罠は、`Display` と同じように `write!` で `Debug` を書くことです。`{:?}` では正しく見えますが、
それ以外はすべて崩れます。`write!(f, "Rgb({:?}, {:?}, {:?})", self.r, self.g, self.b)` の各 `{:?}` は、
呼び出し側の指定ではなく、その文字列に書かれた指定で書式化されます。そのため `{:x?}` でも
`Rgb(255, 0, 16)` と表示され、`{:#?}` でも 1 行のままです。呼び出し側のフラグは `f` に入っていて、
それが見えるのは、フィールドを **`f` で**書式化するコードだけです。

それを行うのがデバッグ用のビルダーで、`#[derive(Debug)]` が展開されるコードもこれです。
`f.debug_struct(name)`（タプルの形なら `f.debug_tuple(name)`）から始め、フィールドごとに `.field(...)` を
呼んで、最後に `.finish()` で終えます。区切りの記号はビルダーが書きます。1 行にするか整形した形にするかは
`f.alternate()` で決まります。各フィールドの `Debug` は同じフォーマッタで呼ばれるので、フィールドにも
同じフラグが届きます。`u8` や `u32` は `{:x?}` で 16 進数になり、`#` もあれば `0x` が付きます。
整形した形では、フィールドが書いた各行をビルダーが字下げします。入れ子のリストやマップが正しく
表示され、値全体を `Vec` に入れても正しく字下げされるのはこのためです。

細かい点が 3 つあります。

- フィールドは、その値の `Debug` で表示されます。`&"<redacted>"` は `&str` なので、クォート付きの
  `"<redacted>"` になります。`format_args!("<redacted>")` は `fmt::Arguments` を作り、その `Debug` は
  文字列をそのまま書きます。
- `finish_non_exhaustive()` は構造体を `..` で閉じます。1 行なら `, .. }`、整形した形なら `..` だけの
  行になります。
- `field` はビルダーを借用した `&mut DebugTuple` を返します。フィールドを付けたり付けなかったり
  するときは、ビルダー自体を `let mut` の変数に入れておきます。
  `let t = f.debug_tuple("Rgba").field(&self.r);` はコンパイルが通りません（E0716）。ビルダーは
  その文の終わりで破棄される一時的な値なのに、`t` がまだそれを借用しているからです。

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
