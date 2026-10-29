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

### Alternatives

The width and the precision can also be given as arguments with `name$`, here read from the
variables `w` and `p` in scope.

```rust
pub fn receipt_line(name: &str, qty: u32, price: f64) -> String {
    let (w, p) = (10, 2);
    format!("{name:<w$.w$} {qty:>3} x {price:>8.p$}")
}
```
