次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

3 つの関数は、どれも答えがわかった時点で調べるのをやめます。

```rust
pub fn find_cell(grid: &[Vec<i32>], target: i32) -> Option<(usize, usize)>
pub fn parse_setting(line: &str) -> Option<(&str, u32)>
pub fn classify(xs: &[i32]) -> &'static str
```

- `find_cell` は、値が `target` であるマスのうち最初のものの位置 `(row, column)`（行、列）を返します。
  グリッドは上の行から順に、各行は左から右へ調べます。添字は 0 から始まります。行ごとに長さが違うことがあり、
  空の行があることもあります。値が `target` のマスがなければ `None` を返します。
- `parse_setting` は `key = value` の形の行を読み取ります。行は**最初の** `=` で分け、その前をキー、後ろを値とします。
  どちらも `str::trim` で前後の空白を取り除きます。結果は `Some((key, value))` で、`key` は空白を取り除いた
  キー（`line` のスライス）、`value` は空白を取り除いた値を `u32` として解釈した数です。
- `parse_setting` は、行に `=` がない場合、空白を取り除いたキーが空の場合、空白を取り除いた値を
  `str::parse::<u32>` が受け付けない場合（空、負の数、数でない、`u32::MAX` より大きいなど）に `None` を返します。
- `classify` は、`xs` が空なら `"empty"` を返します。そうでなければ、0 未満の要素があれば `"negative"`、
  それもなく 0 の要素があれば `"zero"`、どちらもなければ `"positive"` を返します。

シグネチャは変えないでください。

### 例

- `find_cell(&[vec![1, 2], vec![3, 2]], 2)` は `Some((0, 1))`、
  `find_cell(&[vec![1, 2], vec![3, 2]], 4)` は `None` です。
- `parse_setting(" width = 80 ")` は `Some(("width", 80))`、`parse_setting("height")` は `None` です。
- `classify(&[3, 0, 5])` は `"zero"`、`classify(&[4, -2])` は `"negative"` です。
