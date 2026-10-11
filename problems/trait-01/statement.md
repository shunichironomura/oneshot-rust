The `Shape` trait and two shapes, `Rect` and `Circle`, are already declared in the editor. Fill in
the missing bodies.

```rust
pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;
    fn describe(&self) -> String { /* fill in */ }
}

pub struct Rect { pub width: f64, pub height: f64 }
pub struct Circle { pub radius: f64 }

pub fn total_area(shapes: &[&dyn Shape]) -> f64
```

- `area` of a `Rect` is `width * height`. `area` of a `Circle` is π·r² (use
  `std::f64::consts::PI`).
- `name` is `"rect"` for a `Rect` and `"circle"` for a `Circle`.
- `describe` returns `"<name> with area <area>"`, with the area rounded to two decimal places. It
  must also work for other types that implement `Shape`.
- `total_area` returns the sum of the areas of all the shapes (`0.0` for an empty slice).

### Examples

- `Rect { width: 2.0, height: 3.0 }.describe()` is `"rect with area 6.00"`.
- `total_area(&[&Rect { width: 2.0, height: 3.0 }, &Circle { radius: 1.0 }])` is `6.0 + PI`.
