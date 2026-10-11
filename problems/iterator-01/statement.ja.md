`Collatz::new(start)` を、`for` ループや `take`・`sum`・`collect` などのメソッドで使える数列にしてください。

```rust
pub struct Collatz { /* 自由に定義 */ }

impl Collatz {
    pub fn new(start: u64) -> Self;
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item>;
}
```

- 数列は `start` から始まります。値 `n` の次は、`n` が偶数なら `n / 2`、奇数なら `3 * n + 1` です。
- 数列は `1` を出したところで終わります（`1` は含みます）。その後の `next` は `None` を返し続けます。
- `Collatz::new(0)` は空の数列です。
- 値は要求されたときに 1 つずつ計算してください。数列全体を先に作ってはいけません。

### 例

| 式 | 結果 |
| --- | --- |
| `Collatz::new(6).collect::<Vec<_>>()` | `[6, 3, 10, 5, 16, 8, 4, 2, 1]` |
| `Collatz::new(1).collect::<Vec<_>>()` | `[1]` |
