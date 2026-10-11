小さな式言語のトークナイザを書いてください。`tokenize` はソースのテキストをトークンに分けるか、
最初のエラーを報告します。

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Num(u64),
    Ident(String),
    Op(char),
    Arrow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexError {
    Unexpected { ch: char, pos: usize },
    Overflow { pos: usize },
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, LexError>
```

どちらの型もエディタに入っています。テキストは左から右へ読み、各トークンはできるだけ長く取ります。

- ASCII の数字（`0` から `9`）の並びは、その 10 進の値を持つ `Num` です。先頭に 0 があっても
  よく、値は変わりません（`007` は `Num(7)`）。値が `u64::MAX` より大きければ、結果は
  `LexError::Overflow` です。
- 識別子は ASCII 英字か `_` で始まり、ASCII 英字・ASCII 数字・`_` が続きます。その文字列を持つ
  `Ident` になります。数字から始まることはないので、`3x` は `Num(3)` と `Ident("x")` です。
- `->`（`-` のすぐ後に `>`）は `Arrow` です。
- `+ - * / ( ) =` はそれぞれ 1 文字で、その文字を持つ `Op` になります。
- 空白はトークンを区切るだけで、トークンにはなりません。空白とは `char::is_whitespace` が
  `true` を返す文字のことで、U+3000 のような ASCII 以外の空白も含みます。
- それ以外の文字はエラーで、その文字を持つ `LexError::Unexpected` になります。`->` の一部ではない
  `>` や、ASCII 以外の文字（空白を除く。`é` のような英字や数字も含む）もエラーです。
- `pos` は 0 から数えた位置で、バイトではなく文字（`char`）単位で数えます。`Unexpected` では
  その文字の位置、`Overflow` では数の最初の数字の位置です。
- エラーが複数あるときは、最初のもの（`pos` が最も小さいもの）だけを返します。

### 例

結果は `{:?}` で表示したときの形で示します。

```rust
tokenize("x1 -> 42+y") // Ok([Ident("x1"), Arrow, Num(42), Op('+'), Ident("y")])
tokenize("a-b")        // Ok([Ident("a"), Op('-'), Ident("b")])
tokenize("3x")         // Ok([Num(3), Ident("x")])
tokenize("a >= 1")     // Err(Unexpected { ch: '>', pos: 2 })
```
