小さな式言語のトークナイザを書いてください。`tokenize` はソースのテキストをトークンに分けるか、
最初のエラーを報告します。

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Num(u64),
    Op(char),
    Arrow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexError {
    Unexpected { ch: char, pos: usize },
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, LexError>
```

どちらの型もエディタに入っています。テキストは左から右へ読み、各トークンはできるだけ長く取ります。

- ASCII の数字（`0` から `9`）の並びは、その 10 進の値を持つ `Num` です。先頭に 0 があっても
  よく、値は変わりません（`007` は `Num(7)`）。値はいつも `u64` に収まります。
- `->`（`-` のすぐ後に `>`）は `Arrow` です。
- `+ - * / ( ) =` はそれぞれ 1 文字で、その文字を持つ `Op` になります。
- 空白はトークンを区切るだけで、トークンにはなりません。空白とは `char::is_whitespace` が
  `true` を返す文字のことで、U+3000 のような ASCII 以外の空白も含みます。
- それ以外の文字はエラーで、その文字と位置を持つ `LexError::Unexpected` になります。英字、`->` の
  一部ではない `>`、ASCII 以外の文字（空白を除く。`٣`（U+0663）のような数字も含む）もエラーです。
- `pos` は 0 から数えた位置で、バイトではなく文字（`char`）単位で数えます。
- エラーが複数あるときは、最初のもの（`pos` が最も小さいもの）だけを返します。

### 例

結果は `{:?}` で表示したときの形で示します。

```rust
tokenize("1 -> 42+3") // Ok([Num(1), Arrow, Num(42), Op('+'), Num(3)])
tokenize("5-2")       // Ok([Num(5), Op('-'), Num(2)])
tokenize("007*(1)")   // Ok([Num(7), Op('*'), Op('('), Num(1), Op(')')])
tokenize("1 >= 1")    // Err(Unexpected { ch: '>', pos: 2 })
```
