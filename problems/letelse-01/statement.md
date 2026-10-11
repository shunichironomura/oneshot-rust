A shop receives its orders as plain text, one order per line. Read the valid orders, and add up how
many of one item were ordered in total.

```rust
pub fn parse_orders(text: &str) -> Vec<(u32, &str)>
pub fn total_quantity(text: &str, item: &str) -> u32
```

- `text` is split into lines with `str::lines` (a line ends with `\n` or `\r\n`).
- A `#` starts a comment: the `#` and the rest of its line are ignored.
- What is left of a line is a valid order if it consists of **exactly two** words, `COUNT ITEM`.
  Words are separated by whitespace (spaces or tabs, any number of them). Whitespace at the start
  and end of the line does not matter.
- `COUNT` must be a string that `str::parse::<u32>` accepts, and its value must not be `0`. For
  example, `7`, `+7` and `007` all mean 7, while `0`, `-7`, `7.0`, `seven` and `4294967296` (too
  big for a `u32`) are invalid.
- `ITEM` can be any word.
- Lines that are not valid orders, including blank lines and lines that are only a comment, are
  skipped silently.
- `parse_orders` returns the `(count, item)` pair of every valid order, in the order of the lines.
  Each `item` is a slice of `text` (not a copy).
- `total_quantity` returns the sum of the counts of the valid orders whose item is exactly `item`,
  or `0` if there are none. The comparison is case-sensitive: `Cups`, `cup` and `teacups` do not
  match `cups`. You may assume that the sum fits in a `u32`.

### Example

```rust
let text = "3 cups\n# note\n\n 2 pens # red\nx cups\n0 pens\n4 big cups\n5 cups";

parse_orders(text)            // [(3, "cups"), (2, "pens"), (5, "cups")]
total_quantity(text, "cups")  // 8
```

`x cups` has a count that is not a number, `0 pens` has a count of zero, and `4 big cups` has three
words, so these lines are skipped.
