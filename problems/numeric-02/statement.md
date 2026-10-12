Implement seven small integer helpers. Each one is a short formula, but it has to be right for
**every** allowed input, including the largest and smallest values of its types.

```rust
pub fn sum_wrapping(xs: &[u8]) -> u8
pub fn sum_checked(xs: &[u8]) -> Option<u8>
pub fn charge(level: u8, amount: u8) -> u8
pub fn wrap_index(i: i64, len: usize) -> usize
pub fn page_count(items: u64, per_page: u64) -> u64
pub fn middle(a: u64, b: u64) -> u64
pub fn distance(a: i64, b: i64) -> u64
```

The formulas below are meant mathematically, as if computed with integers of unlimited size, and
every result fits in the return type. No function may panic for any input allowed here.

- `sum_wrapping(xs)` returns the sum of `xs` modulo 256. It is `0` for an empty slice.
- `sum_checked(xs)` returns the sum of `xs` in `Some` if it is at most 255, and `None` otherwise.
  The sum of an empty slice is `0`, so the result is `Some(0)`.
- `charge(level, amount)` charges a battery at `level` percent by `amount` more percentage points.
  It returns `level + amount`, or 100 if that is more than 100. `level` is at most 100, and
  `amount` can be any `u8`.
- `wrap_index(i, len)` returns `i` modulo `len` as a value in `0..len`: the `r` in `0..len` such
  that `i - r` is a multiple of `len`. Think of `len` slots in a ring, counted in both directions,
  so `-1` is the last slot. `len` is between 1 and 1,000,000.
- `page_count(items, per_page)` returns how many pages are needed to show `items` items with at
  most `per_page` items per page, that is, `items / per_page` rounded up. Zero items need zero
  pages. `per_page` is at least 1.
- `middle(a, b)` returns `(a + b) / 2` rounded down.
- `distance(a, b)` returns the absolute difference `|a - b|`.

### Examples

- `sum_wrapping(&[200, 100])` is `44`, `sum_checked(&[200, 100])` is `None`, and
  `sum_checked(&[100, 155])` is `Some(255)`.
- `charge(50, 20)` is `70`, and `charge(90, 200)` is `100`.
- `wrap_index(-1, 5)` is `4`, and `wrap_index(12, 5)` is `2`.
- `page_count(10, 3)` is `4`.
- `middle(u64::MAX, u64::MAX - 2)` is `u64::MAX - 1`.
- `distance(i64::MIN, i64::MAX)` is `u64::MAX`.
