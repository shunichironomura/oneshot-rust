The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

`moving_average` returns the average of every run of `window` consecutive values, from left to
right. If `window` is `0` or larger than the number of values, it returns an empty vector. The values
can be anywhere in the `i32` range.

```rust
pub fn moving_average(values: &[i32], window: usize) -> Vec<f64>
```

Do not change the signature.

### Examples

| `values` | `window` | Output |
| --- | --- | --- |
| `[1, 2, 3, 4, 5]` | `2` | `[1.5, 2.5, 3.5, 4.5]` |
| `[1, 2]` | `3` | `[]` |
