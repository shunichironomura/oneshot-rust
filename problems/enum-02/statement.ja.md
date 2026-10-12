表計算ソフトの 1 行が、カンマ区切りのテキストとして届きます。下の列挙型 `Cell`（エディタに入っています）を使って
型の付いたセルに変換し、行から 2 つの値を求めてください。

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Int(i64),
    Text(String),
    Empty,
}

pub fn parse_row(line: &str) -> Vec<Cell>
pub fn numeric_sum(row: &[Cell]) -> i64
pub fn texts(row: &[Cell]) -> Vec<&str>
```

- `parse_row` は `line` をすべての `,` で区切り、フィールドごとに 1 つのセルを順に返します。空の行にも
  フィールドが 1 つあります。カンマが *n* 個ある行からは、必ず *n* + 1 個のセルができます。
- 各フィールドは、まず前後の空白を取り除きます（`str::trim` と同じです）。取り除いたあとのフィールドが
  空なら `Empty` です。そうでなく、`field.parse::<i64>()` が `Ok(n)` を返すなら `Int(n)`、それ以外は
  そのフィールドを持つ `Text` です。
- `numeric_sum` は、`Int` のセルの値をすべて足した合計を返します。`Int` のセルがなければ `0` です。
  テストでは、合計は `i64` の範囲に収まります。
- `texts` は、`Text` のセルの文字列を順に、`row` から借用したスライスとして返します。

### 例

```rust
let row = parse_row("3, +5, 1.5,  blue sky , ");
// [Int(3), Int(5), Text("1.5"), Text("blue sky"), Empty]

numeric_sum(&row) // 8
texts(&row)       // ["1.5", "blue sky"]
parse_row("")     // [Empty]
```
