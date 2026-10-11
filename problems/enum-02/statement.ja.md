表計算ソフトの 1 行が、カンマ区切りのテキストとして届きます。下の列挙型 `Cell`（エディタに入っています）を使って
型の付いたセルに変換し、行から 2 つの値を求めてください。

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Int(i64),
    Float(f64),
    Text(String),
    Empty,
}

pub fn parse_row(line: &str) -> Vec<Cell>
pub fn numeric_sum(row: &[Cell]) -> f64
pub fn texts(row: &[Cell]) -> Vec<&str>
```

- `parse_row` は `line` をすべての `,` で区切り、フィールドごとに 1 つのセルを順に返します。空の行にも
  フィールドが 1 つあります。カンマが *n* 個ある行からは、必ず *n* + 1 個のセルができます。
- 各フィールドは、まず前後の空白を取り除きます（`str::trim` と同じです）。取り除いたあとのフィールドを
  次の表の上から順に調べ、最初に当てはまった行のセルにします。

| 前後の空白を除いたフィールド | セル |
| --- | --- |
| 空 | `Empty` |
| `field.parse::<i64>()` が `Ok(n)` を返す | `Int(n)` |
| `field.parse::<f64>()` が `Ok(x)` を返す | `Float(x)` |
| それ以外 | そのフィールドを持つ `Text` |

何が数になるかは、この 2 つの標準のパーサーだけで決まります。`.5` や `inf` のような、あまり見かけない形も
パーサーが受け付けるなら数です。テストには、NaN になるフィールドはありません。

- `numeric_sum` は、`Int` と `Float` のセルの値をすべて足した合計を返します。各値は `f64` に変換してから
  足すので、大きな整数でも `i64` のオーバーフローは起きません。数のセルがなければ `0.0` です。
- `texts` は、`Text` のセルの文字列を順に、`row` から借用したスライスとして返します。

### 例

```rust
let row = parse_row("3, +5, 1e3, -0.5,  blue sky , ");
// [Int(3), Int(5), Float(1000.0), Float(-0.5), Text("blue sky"), Empty]

numeric_sum(&row) // 1007.5
texts(&row)       // ["blue sky"]
parse_row("")     // [Empty]
```
