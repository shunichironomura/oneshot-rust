次のコードはコンパイルが通りません。直してください。

小さな CSV ファイルの各行には、ID、年齢、スコアの 3 つの列があるはずです。`validate_row` は行を読み取れるかどうかを
確かめ（値そのものは使いません）、`sum_ids` は複数の行の ID を合計します。`BadField` は読み取れなかった列を表します
（エディタに入っています）。

```rust
#[derive(Debug, PartialEq)]
pub struct BadField {
    pub column: usize,
    pub text: String,
}

pub fn validate_row(line: &str) -> Result<(), BadField>;
pub fn sum_ids(lines: &[&str]) -> Result<u64, BadField>;
```

- 行はすべての `,` で区切ります。列は 0 から数え、列 0 が ID、列 1 が年齢、列 2 がスコアです。列 2 より後ろの列は
  無視し、足りない列は空の文字列 `""` として扱います。
- 各列の文字列は `str::trim` で前後の空白を取り除いてから、`str::parse` で解釈します。ID は `u32`、年齢は `u8`、
  スコアは `f64` として解釈します。
- `validate_row` は列 0、1、2 の順に調べます。最初に解釈できなかった列について `Err(BadField { column, text })`
  を返します。`text` はその列の空白を取り除いた後の文字列です。3 つとも解釈できれば `Ok(())` を返します。
- `sum_ids` は行を順に `validate_row` と同じ規則で調べます。正しくない行があれば、最初の正しくない行のエラーを
  返します。そうでなければ ID の合計を `Ok` で返します。合計は `u32::MAX` を超えることがあります（空のリストなら
  `Ok(0)`）。

公開されているシグネチャは変えないでください。非公開のヘルパー `field` は変更・削除して構いません。

### 例

- `validate_row("7, 30, 1.5")` は `Ok(())` です。
- `validate_row("7, 300, 1.5")` は `Err(BadField { column: 1, text: "300".to_string() })` です。300 は `u8` に
  収まらないからです。
- `validate_row("7")` は `Err(BadField { column: 1, text: "".to_string() })` です（年齢がありません）。
- `sum_ids(&["7, 30, 1.5", "35, 41, 0.25"])` は `Ok(42)` です。
