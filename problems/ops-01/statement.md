`Vec2<T>` is a 2D vector whose components can be any number type. Make the usual vector arithmetic
work on it with the ordinary operators. The harness uses it with integers (`i32`, `i64`) and
floating-point numbers (`f32`, `f64`).

| Code | Result |
| --- | --- |
| `a + b` | `Vec2 { x: a.x + b.x, y: a.y + b.y }` |
| `a += b` | same as `a = a + b` |
| `a * k` (`k: T`) | `Vec2 { x: a.x * k, y: a.y * k }` |
| `-a` | `Vec2 { x: -a.x, y: -a.y }` |
| `a.dot(b)` | `a.x * b.x + a.y * b.y` |

`template.rs` already lays out the `impl` blocks. Do not change the definition of `Vec2`; anything
else may be changed as needed.

### Examples

With `a = Vec2::new(1, 2)` and `b = Vec2::new(3, -4)`: `a + b` is `Vec2::new(4, -2)`, `a * 3` is
`Vec2::new(3, 6)`, `-b` is `Vec2::new(-3, 4)` and `a.dot(b)` is `-5`.

After `let mut p = Vec2::new(0.5, 1.0); p += Vec2::new(0.25, -2.0);`, `p` is
`Vec2::new(0.75, -1.0)` and `-(p * 2.0)` is `Vec2::new(-1.5, 2.0)`.
