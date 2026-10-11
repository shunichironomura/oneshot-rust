次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

5 つの関数は、どれも `match` を使って入力をいくつかの場合に分けます。

```rust
pub fn digits_label(n: u8) -> &'static str;
pub fn sign_word(n: i32) -> &'static str;
pub fn quadrant(x: i32, y: i32) -> &'static str;
pub fn grade(score: u32) -> char;
pub fn parse_answer(s: &str) -> Option<bool>;
```

- `digits_label` は `n` が 10 進数で何桁かを返します。0 から 9 は `"one digit"`、10 から 99 は `"two digits"`、
  100 から 255 は `"three digits"` です。
- `sign_word` は `n` の符号に応じて `"negative"`、`"zero"`、`"positive"` のどれかを返します。
- `quadrant` は点 `(x, y)` がどこにあるかを返します。`x > 0` かつ `y > 0` なら `"I"`、`x < 0` かつ `y > 0` なら
  `"II"`、`x < 0` かつ `y < 0` なら `"III"`、`x > 0` かつ `y < 0` なら `"IV"` です。軸の上の点はどの象限にも
  入りません。`(0, 0)` は `"origin"`、それ以外で `x == 0` の点は `"y-axis"`、それ以外で `y == 0` の点は
  `"x-axis"` です。
- `grade` は点数を評価の文字に変えます。90 以上は `'A'`、80 から 89 は `'B'`、70 から 79 は `'C'`、60 から 69 は
  `'D'`、60 未満は `'F'` です。加点で 100 を超えることもありますが、その場合も `'A'` です。
- `parse_answer` は yes / no の答えを読み取ります。`"yes"` と `"y"` は `Some(true)`、`"no"` と `"n"` は
  `Some(false)`、それ以外の文字列はすべて `None` です。比較は完全一致で、大文字・小文字をそろえたり空白を
  取り除いたりはしません。たとえば `"Yes"` や `" y"` は `None` です。

シグネチャは変えないでください。

### 例

| 呼び出し | 結果 |
| --- | --- |
| `digits_label(42)` | `"two digits"` |
| `sign_word(-7)` | `"negative"` |
| `quadrant(-2, 5)` | `"II"` |
| `quadrant(0, -3)` | `"y-axis"` |
| `grade(85)` | `'B'` |
| `parse_answer("y")` | `Some(true)` |
| `parse_answer("maybe")` | `None` |
