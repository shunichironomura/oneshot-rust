次のコードはコンパイルが通りません。直してください。

3 つの関数は、どれも `match` を使って入力をいくつかの場合に分けます。

```rust
pub fn digits_label(n: u8) -> &'static str;
pub fn sign_word(n: i32) -> &'static str;
pub fn quadrant(x: i32, y: i32) -> &'static str;
```

- `digits_label` は `n` が 10 進数で何桁かを返します。0 から 9 は `"one digit"`、10 から 99 は `"two digits"`、
  100 から 255 は `"three digits"` です。
- `sign_word` は `n` の符号に応じて `"negative"`、`"zero"`、`"positive"` のどれかを返します。
- `quadrant` は点 `(x, y)` がどこにあるかを返します。`x > 0` かつ `y > 0` なら `"I"`、`x < 0` かつ `y > 0` なら
  `"II"`、`x < 0` かつ `y < 0` なら `"III"`、`x > 0` かつ `y < 0` なら `"IV"` です。軸の上の点はどの象限にも
  入りません。`(0, 0)` は `"origin"`、それ以外で `x == 0` の点は `"y-axis"`、それ以外で `y == 0` の点は
  `"x-axis"` です。

シグネチャは変えないでください。

### 例

| 呼び出し | 結果 |
| --- | --- |
| `digits_label(42)` | `"two digits"` |
| `sign_word(-7)` | `"negative"` |
| `quadrant(-2, 5)` | `"II"` |
| `quadrant(0, -3)` | `"y-axis"` |
