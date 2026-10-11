Implement three functions that take arrays and return arrays. The lengths are const generic
parameters, so each function works for arrays of any length.

```rust
pub fn shout<const N: usize>(names: [String; N]) -> [String; N]
pub fn transpose<const R: usize, const C: usize>(m: [[i32; C]; R]) -> [[i32; R]; C]
pub fn zip_arrays<A, B, const N: usize>(a: [A; N], b: [B; N]) -> [(A, B); N]
```

- `shout` converts every name to uppercase with `str::to_uppercase` and appends `!` to it. Each
  result stays at the position of its name. `to_uppercase` follows Unicode rules, so non-ASCII
  letters are converted too, and a name can get longer (`"ß"` becomes `"SS"`).
- `transpose` returns the transpose of the matrix `m`, which has `R` rows of `C` numbers each. The
  result has `C` rows of `R` numbers each, and `result[c][r] == m[r][c]`.
- `zip_arrays` pairs up the elements at the same position: `result[i]` is `(a[i], b[i])`. `A` and
  `B` can be any types, including types that implement neither `Copy` nor `Clone`.
- All three functions must also work when a length is zero (`N = 0`, `R = 0` or `C = 0`). For
  example, `transpose::<0, 3>([])` returns three empty rows.

### Examples

- `shout(["ann".to_string(), "bob".to_string()])` returns `["ANN!", "BOB!"]` (as `String`s).
- `transpose([[1, 2, 3], [4, 5, 6]])` returns `[[1, 4], [2, 5], [3, 6]]`.
- `zip_arrays([1, 2], ["a", "b"])` returns `[(1, "a"), (2, "b")]`.
