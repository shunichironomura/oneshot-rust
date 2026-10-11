次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

小さなデータファイルの各行には、整数 `a` と小数 `b` の組 `a,b` が書かれています。
`parse_pair` は 1 組を読み取り、`sum_pairs` はファイル中のすべての組を合計します。
エラーは次の `AppError` 型で報告します（`Display` と `Error` の実装も含めて、
エディタに入っています）。

```rust
#[derive(Debug, PartialEq)]
pub enum AppError {
    MissingComma,
    Parse(String),
}

impl fmt::Display for AppError { /* "missing comma" / "parse error: <msg>" */ }
impl Error for AppError {}

pub fn parse_pair(s: &str) -> Result<(i32, f64), AppError>
pub fn sum_pairs(text: &str) -> Result<f64, Box<dyn Error>>
```

- `parse_pair` は `s` を**最初の** `,` で 2 つに分け、それぞれの前後の空白を
  （`str::trim` と同じように）取り除きます。そのうえで `str::parse` を使い、`a` を `i32` として、
  `b` を `f64` として解釈します。
- `s` に `,` がなければ、結果は `Err(AppError::MissingComma)` です。
- 解釈できない部分があれば、結果は `Err(AppError::Parse(msg))` です。`msg` は、`parse` が返した
  `ParseIntError` または `ParseFloatError` の `to_string()` です。両方とも不正なときは
  `a` のエラーにします。
- `AppError` の表示は `missing comma` と `parse error: <msg>` です。
- `sum_pairs` は `text` の行（`str::lines` で分けたもの）を順に処理し、空行と空白だけの行は
  読み飛ばします。それ以外の行はすべて `parse_pair` で読み取り、`a as f64 + b` の合計を返します
  （組が 1 つもなければ `0.0`）。
- `parse_pair` が受け付けない行があれば、`sum_pairs` は最初のその行で処理をやめ、その `AppError` を
  ボックス化して返します。テストでは `downcast_ref::<AppError>()` で取り出します。

`AppError` とその `Display`・`Error` の実装、`parse_pair` と `sum_pairs` のシグネチャは
変えないでください。それ以外は変更して構いません。

### 例

```rust
parse_pair("3, 0.5")          // Ok((3, 0.5))
parse_pair("x,1")             // Err(AppError::Parse("invalid digit found in string".to_string()))
parse_pair("1,abc")           // Err(AppError::Parse("invalid float literal".to_string()))

sum_pairs("1,0.5\n\n2,1.5")   // Ok(5.0)
sum_pairs("1,1\n2")           // Err(..)。AppError::MissingComma で、表示は "missing comma"
```
