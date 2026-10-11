Colors on the web are often written as hex codes such as `#ff8800`. Implement a function that reads
such a code and one that writes it.

```rust
pub fn parse_hex_color(s: &str) -> Option<[u8; 3]>
pub fn to_hex(rgb: [u8; 3]) -> String
```

A *hex digit* is one of the ASCII characters `0` to `9`, `a` to `f` and `A` to `F`, with a value
from 0 to 15 (`a` and `A` are 10, `f` and `F` are 15). Upper and lower case may be mixed freely.

- `parse_hex_color` returns the red, green and blue values of the color `s`, in this order. It
  accepts exactly two forms:
  - `#` followed by 6 hex digits, `rrggbb`. Each channel is a two-digit hex number, so `#0A0b0C`
    is `[10, 11, 12]`.
  - `#` followed by 3 hex digits, `rgb`. Each digit `d` stands for the two digits `dd`, so `#f80`
    means `#ff8800`, that is `[255, 136, 0]`.
- `s` must be exactly one of these two forms, with nothing else before, inside or after it.
  Everything else gives `None`, for example: no `#` at the start, a second `#`, any other number of
  digits (CSS also has 4- and 8-digit forms with transparency, but they are not accepted here),
  whitespace anywhere (`s` is not trimmed), or any other character that is not a hex digit,
  ASCII or not.
- `parse_hex_color` must not panic, whatever `s` is.
- `to_hex` returns the shortest code that `parse_hex_color` reads back as `rgb`, written with
  lowercase digits:
  - If each of the three channels has two equal hex digits (it is one of `0x00`, `0x11`, ...,
    `0xff`), the result is the 3-digit form: `[255, 136, 0]` gives `"#f80"`.
  - Otherwise the result is the 6-digit form, with exactly two digits for every channel:
    `[255, 8, 0]` gives `"#ff0800"`.

### Examples

| Call | Result |
| --- | --- |
| `parse_hex_color("#0A0b0C")` | `Some([10, 11, 12])` |
| `parse_hex_color("#f80")` | `Some([255, 136, 0])` |
| `parse_hex_color("ff8800")` | `None` (no `#`) |
| `parse_hex_color("#ff80")` | `None` (4 digits) |
| `to_hex([255, 8, 0])` | `"#ff0800"` |
| `to_hex([255, 136, 0])` | `"#f80"` |
