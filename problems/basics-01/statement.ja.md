次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

数を扱う小さな関数が 3 つ入っています。

```rust
pub fn digital_root(n: u64) -> u64
pub fn fizz_label(n: u32) -> String
pub fn first_multiple_above(step: u64, limit: u64) -> u64
```

- `digital_root(n)` は、`n` を「10 進数の各桁の和」で置き換えることを、10 未満になるまで繰り返し、
  その数を返します。最初から 10 未満の数（`0` を含む）はそのまま返します。
- `fizz_label(n)` は、`n` が 15 の倍数なら `"FizzBuzz"`、そうでなく 3 の倍数なら `"Fizz"`、
  そうでなく 5 の倍数なら `"Buzz"`、どれでもなければ `n` を 10 進数で書いた文字列を返します。
  `0` はどの数の倍数でもあるものとします。
- `first_multiple_above(step, limit)` は、`step` の倍数のうち `limit` より**大きい**最小のものを返します
  （`limit` と等しいものは含みません）。`step` は 1 以上で、答えは必ず `u64` に収まります。

シグネチャは変えないでください。

### 例

- `digital_root(9875)` は `2` です（9875 → 29 → 11 → 2）。
- `fizz_label(10)` は `"Buzz"`、`fizz_label(7)` は `"7"` です。
- `first_multiple_above(7, 20)` は `21`、`first_multiple_above(5, 10)` は `15`、
  `first_multiple_above(3, 0)` は `3` です。
