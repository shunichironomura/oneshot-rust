関数 `k_smallest` を実装してください。

```rust
pub fn k_smallest(v: &[i32], k: usize) -> Vec<i32>
```

- `k_smallest` は `v` の値のうち小さいほうから `k` 個を、昇順に並べて返します。重複した値もそのまま残します。
- `v` にはどんな `i32` の値も入りえます（`i32::MIN` や `i32::MAX` も）。`k` もどんな `usize` でもかまいません。
- `k` が `v.len()` 以上なら、`v` のすべての値を昇順に並べて返します。

`std` のものは何を使ってもかまいません（たとえば `std::collections::BinaryHeap`）。

### 例

`k_smallest(&[5, 1, 4, 1, 3], 3)` は `[1, 1, 3]` を返します。
