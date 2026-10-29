A graphics library writes colors to its log with `{:?}`, and a fully opaque color should not clutter
the log with its alpha channel. Implement `Debug` by hand for `Color` (it is already in the editor).

```rust
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl fmt::Debug for Color { /* to implement */ }
```

- `Color` prints exactly what `#[derive(Debug)]` would print for the tuple struct `Rgb(r, g, b)`
  when `a` is 255 (fully opaque), and for the tuple struct `Rgba(r, g, b, a)` otherwise.
- Like a derived `Debug`, the output follows the formatter's flags: the pretty form of `{:#?}`,
  the hexadecimal integers of `{:x?}` and `{:X?}` (with `0x` under `#`), and the extra indentation
  when the value is nested inside another value printed with `{:#?}`.
- Width, fill, alignment, precision and the `+` and `0` flags are not used.

### Examples

With `red = Color { r: 255, g: 0, b: 16, a: 255 }` and
`glass = Color { r: 255, g: 0, b: 16, a: 128 }`:

| Code | Result |
| --- | --- |
| `format!("{red:?}")` | `Rgb(255, 0, 16)` |
| `format!("{red:x?}")` | `Rgb(ff, 0, 10)` |
| `format!("{glass:?}")` | `Rgba(255, 0, 16, 128)` |

and `format!("{red:#X?}")` is

```text
Rgb(
    0xFF,
    0x0,
    0x10,
)
```
