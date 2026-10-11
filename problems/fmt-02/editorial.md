Everything here comes down to the format spec mini-language,
`{arg:[fill][align][#][0][width][.precision][type]}`. In the solution, each part of a spec has one
job.

- `{name:<10.10}`: the width `10` is a **minimum**, filled with spaces. On a string, the precision
  `.10` is a **maximum** number of characters, so a longer name is cut. Without it, `Chocolate bar`
  would push the rest of the line to the right.
- `{qty:>3}` and `{price:>8.2}`: on a float, the precision is the number of digits after the point,
  and the value is rounded (`19.999` becomes `20.00`). A width never cuts anything, so `1000` and
  `100000.00` come out in full. Numbers are right-aligned by default and strings left-aligned, so
  `{qty:3}` and `{price:8.2}` work too.
- `{r:02X}`: the `0` flag pads with zeros instead of spaces, `2` is the width and `X` is uppercase
  hexadecimal. `{r:X}` prints `3` as a single `3`, which makes the color too short, and `{r:2X}`
  pads with a space.
- `{n:#010b}`: `#` adds the `0b` prefix, and the width counts the prefix too, so 8 digits need a
  width of 10. `{n:#08b}` is a common guess, and it gives only six digits: `0b000101`.
- `{cell:^width$}`: a width known only at run time is written `name$`, which reads it from the
  argument (or the variable in scope) called `name`. `{cell:^width}` does not compile, because
  `width` is read as a format trait. Center alignment already puts the odd space on the right.

Starting with `"|"` and appending `cell|` for each cell also gives the right row for an empty slice,
whereas collecting the centered cells into `parts` and writing `format!("|{}|", parts.join("|"))`
would give `||` there.

### Alternatives

Write the `0b` by hand; then the width counts only the digits.

```rust
pub fn bits(n: u8) -> String {
    format!("0b{n:08b}")
}
```

`1$` takes the width from the positional argument at index 1. `write!` can also write straight into
the `String`, without building a temporary `String` per cell. Writing to a `String` never fails, so
the `fmt::Result` can be unwrapped.

```rust
use std::fmt::Write;

pub fn table_row(cells: &[&str], width: usize) -> String {
    let mut row = String::from("|");
    for cell in cells {
        write!(row, "{:^1$}|", cell, width).unwrap();
    }
    row
}
```

Centering by hand shows what `^` does. `saturating_sub` keeps a cell longer than `width` from
underflowing, and `padding / 2` rounds down, so the extra space ends up on the right.

```rust
pub fn table_row(cells: &[&str], width: usize) -> String {
    let mut row = String::from("|");
    for cell in cells {
        let padding = width.saturating_sub(cell.len());
        let left = padding / 2;
        row.push_str(&" ".repeat(left));
        row.push_str(cell);
        row.push_str(&" ".repeat(padding - left));
        row.push('|');
    }
    row
}
```
