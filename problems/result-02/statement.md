Each function below handles a whole batch of values at once, and some of the values may be invalid
or missing. Implement all four.

```rust
use std::num::ParseIntError;

pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError>
pub fn split_parsed<'a>(items: &[&'a str]) -> (Vec<i32>, Vec<&'a str>)
pub fn double_first(items: &[&str]) -> Result<Option<i64>, ParseIntError>
pub fn sum_all(xs: &[Option<u32>]) -> Option<u32>
```

- An item is **valid** if `item.parse::<i32>()` succeeds, and its value is the parsed number. Items
  are parsed exactly as given: whitespace is not trimmed. The error of an invalid item is the
  `ParseIntError` that `item.parse::<i32>()` returns.
- `parse_all` returns `Ok` with the values of all the items, in input order, if every item is valid
  (an empty `Vec` for an empty slice). Otherwise it returns `Err` with the error of the **first**
  invalid item.
- `split_parsed` returns the values of the valid items and the invalid items themselves, both in
  input order.
- `double_first` looks only at the first item and ignores the others, valid or not. It returns
  `Ok(None)` for an empty slice, `Ok(Some(m))` if the first item is valid, where `m` is twice its
  value, and `Err` with the error of the first item otherwise.
- `sum_all` returns `Some` with the sum of the numbers if every element is `Some` and the sum is at
  most `u32::MAX`. Otherwise (an element is `None`, or the sum is greater than `u32::MAX`) it returns
  `None`. The sum of an empty slice is `0`, so the result is `Some(0)`.
- No function may panic, whatever the input.

### Examples

```rust
parse_all(&["7", "-3"])                  // Ok(vec![7, -3])
parse_all(&["1", "x", ""])               // Err(e), where e is the error of "x" (an invalid digit),
                                         // not the error of "" (an empty string)
split_parsed(&["4", "a", "-2", "1.5"])   // (vec![4, -2], vec!["a", "1.5"])
double_first(&["21", "x"])               // Ok(Some(42))
sum_all(&[Some(1), Some(2)])             // Some(3)
sum_all(&[Some(u32::MAX), Some(1)])      // None
```
