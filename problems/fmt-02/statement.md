A small shop prints its receipts on a fixed-width printer. Implement `receipt_line`, which lays out
one line of a receipt.

```rust
pub fn receipt_line(name: &str, qty: u32, price: f64) -> String
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
- `name` is ASCII.

### Examples

```rust
receipt_line("Coffee", 2, 3.5)               // "Coffee       2 x     3.50"
receipt_line("Chocolate bar", 12, 1234.567)  // "Chocolate   12 x  1234.57"
```
