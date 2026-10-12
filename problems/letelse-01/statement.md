A shop receives its orders as plain text, one order per line. Read the valid orders.

```rust
pub fn parse_orders(text: &str) -> Vec<(u32, &str)>
```

- `text` is split into lines with `str::lines` (a line ends with `\n` or `\r\n`).
- A line is a valid order if it consists of **exactly two** words, `COUNT ITEM`. Words are
  separated by whitespace (spaces or tabs, any number of them). Whitespace at the start and end of
  the line does not matter.
- `COUNT` must be a string that `str::parse::<u32>` accepts, and its value must not be `0`. For
  example, `7`, `+7` and `007` all mean 7, while `0`, `-7`, `7.0`, `seven` and `4294967296` (too
  big for a `u32`) are invalid.
- `ITEM` can be any word.
- Lines that are not valid orders, including blank lines, are skipped silently.
- `parse_orders` returns the `(count, item)` pair of every valid order, in the order of the lines.
  Each `item` is a slice of `text` (not a copy).

### Example

```rust
let text = "3 cups\n\n 2 pens\nx cups\n0 pens\n4 big cups\n5 cups";

parse_orders(text)  // [(3, "cups"), (2, "pens"), (5, "cups")]
```

`x cups` has a count that is not a number, `0 pens` has a count of zero, and `4 big cups` has three
words, so these lines are skipped.
