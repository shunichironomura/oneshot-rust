Implement the function `parse_all`, which parses a whole batch of numbers at once. Some of the items
may be invalid.

```rust
use std::num::ParseIntError;

pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError>
```

- An item is **valid** if `item.parse::<i32>()` succeeds, and its value is the parsed number. Items
  are parsed exactly as given: whitespace is not trimmed. The error of an invalid item is the
  `ParseIntError` that `item.parse::<i32>()` returns.
- `parse_all` returns `Ok` with the values of all the items, in input order, if every item is valid
  (an empty `Vec` for an empty slice). Otherwise it returns `Err` with the error of the **first**
  invalid item.

### Examples

```rust
parse_all(&["7", "-3"])                  // Ok(vec![7, -3])
parse_all(&["1", "x", ""])               // Err(e), where e is the error of "x" (an invalid digit),
                                         // not the error of "" (an empty string)
```
