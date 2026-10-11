A small shop prints its receipts on a fixed-width printer. Implement four helpers that lay out the
text.

```rust
pub fn receipt_line(name: &str, qty: u32, price: f64) -> String
pub fn hex_color(r: u8, g: u8, b: u8) -> String
pub fn bits(n: u8) -> String
pub fn table_row(cells: &[&str], width: usize) -> String
```

- `receipt_line` joins these parts in this order, with nothing in between:
  1. `name`, left-aligned in a field of 10 characters (filled with spaces on the right). A name
     longer than 10 characters is cut to its first 10 characters.
  2. One space.
  3. `qty`, right-aligned in a field of 3 characters (filled with spaces on the left). A wider
     number is printed in full.
  4. `" x "` (a space, `x` and a space).
  5. `price` rounded to the nearest hundredth and always written with exactly two digits after the
     point, right-aligned in a field of 8 characters (filled with spaces on the left). A wider
     price is printed in full. `price` is finite and non-negative, and never exactly halfway
     between two hundredths.
- `hex_color` returns `#` followed by `r`, `g` and `b`, in this order, each written as exactly two
  uppercase hexadecimal digits.
- `bits` returns `0b` followed by `n` written as exactly 8 binary digits.
- `table_row` centers each cell in a field of `width` characters, filled with spaces. When the
  spaces cannot be split evenly, the extra space goes on the right. A cell longer than `width` is
  printed in full, with no spaces added. Each cell is preceded by `|`, and one more `|` ends the
  row, so a row with no cells is just `|`.
- All strings are ASCII.

### Examples

```rust
receipt_line("Coffee", 2, 3.5)               // "Coffee       2 x     3.50"
receipt_line("Chocolate bar", 12, 1234.567)  // "Chocolate   12 x  1234.57"
hex_color(128, 255, 90)                      // "#80FF5A"
bits(5)                                      // "0b00000101"
table_row(&["a", "bb"], 4)                   // "| a  | bb |"
```
