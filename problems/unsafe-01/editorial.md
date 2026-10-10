`push` may reallocate the buffer of `v` and move its elements, which leaves the raw pointer `first`
dangling. Even without a reallocation, mutating `v` through `&mut` invalidates pointers derived
earlier, and Miri's Stacked Borrows check reports that.

`i32` is `Copy`, so the fix is simply to read the value **before** the push. No `unsafe` is needed.

### Alternatives

Calculating the result without `push` first, then pushing, also avoids holding anything across the
mutation.

```rust
pub fn push_and_sum(v: &mut Vec<i32>, extra: i32) -> i32 {
    let result = v.iter().sum::<i32>() + extra + v[0];
    v.push(extra);
    result
}
```
