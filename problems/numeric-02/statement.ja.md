整数を扱う小さな関数を 7 つ実装してください。どれも短い式ですが、型の最大値や最小値も含めて、
許された**すべての**入力に対して正しく動かなければなりません。

```rust
pub fn sum_wrapping(xs: &[u8]) -> u8
pub fn sum_checked(xs: &[u8]) -> Option<u8>
pub fn charge(level: u8, amount: u8) -> u8
pub fn wrap_index(i: i64, len: usize) -> usize
pub fn page_count(items: u64, per_page: u64) -> u64
pub fn middle(a: u64, b: u64) -> u64
pub fn distance(a: i64, b: i64) -> u64
```

以下の式は数学的な意味で、大きさに上限のない整数で計算した値を表します。結果はどれも戻り値の型に収まります。
どの関数も、ここで許された入力に対してパニックしてはいけません。

- `sum_wrapping(xs)` は `xs` の合計を 256 で割った余りを返します。空のスライスなら `0` です。
- `sum_checked(xs)` は、`xs` の合計が 255 以下ならその合計を `Some` に入れて返し、そうでなければ
  `None` を返します。空のスライスの合計は `0` なので、結果は `Some(0)` です。
- `charge(level, amount)` は、残量が `level` パーセントの電池を、さらに `amount` パーセントポイント
  充電します。`level + amount` を返しますが、100 を超えるときは 100 を返します。`level` は 100 以下で、
  `amount` は `u8` のどの値にもなりえます。
- `wrap_index(i, len)` は `i` を `len` で割った余りを `0..len` の範囲で返します。つまり、`i - r` が
  `len` の倍数になる `0..len` の中の `r` です。`len` 個のスロットが輪になって並んでいて、
  どちらの向きにも数えられると考えてください。`-1` は最後のスロットです。`len` は 1 以上
  1,000,000 以下です。
- `page_count(items, per_page)` は、1 ページに最大 `per_page` 個ずつ表示するとき、`items` 個を表示するのに
  必要なページ数、つまり `items / per_page` を切り上げた値を返します。0 個なら 0 ページです。`per_page` は
  1 以上です。
- `middle(a, b)` は `(a + b) / 2` を切り捨てた値を返します。
- `distance(a, b)` は差の絶対値 `|a - b|` を返します。

### 例

- `sum_wrapping(&[200, 100])` は `44`、`sum_checked(&[200, 100])` は `None`、
  `sum_checked(&[100, 155])` は `Some(255)` です。
- `charge(50, 20)` は `70`、`charge(90, 200)` は `100` です。
- `wrap_index(-1, 5)` は `4`、`wrap_index(12, 5)` は `2` です。
- `page_count(10, 3)` は `4` です。
- `middle(u64::MAX, u64::MAX - 2)` は `u64::MAX - 1` です。
- `distance(i64::MIN, i64::MAX)` は `u64::MAX` です。
