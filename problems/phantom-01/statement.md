The code below does not compile. Fix it.

`Length<U>` is a length whose unit is part of its type: `Length<Mm>` and `Length<Inch>` are
different types, so millimeters and inches cannot be mixed up by mistake. A unit type holds no data;
a `Length<U>` stores only an `f64` and a zero-sized `PhantomData<U>`. Lengths work with the usual
arithmetic operators, and a length can be multiplied by a number from either side.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mm;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Inch;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length<U> {
    value: f64,
    unit: PhantomData<U>,
}

impl<U> Length<U> {
    pub fn new(value: f64) -> Self;
    pub fn value(self) -> f64;
}

pub fn perimeter<U>(w: Length<U>, h: Length<U>) -> Length<U>
pub fn to_mm(l: Length<Inch>) -> Length<Mm>
```

- `new` creates a length with the given value, and `value` returns the value. Two lengths of the
  same unit are equal (`==`) when their values are equal.
- For two lengths `a` and `b` of the same unit, `a + b` is the length whose value is the sum of
  their values, and `a / b` is the quotient of their values, a plain `f64` (how many times `b` fits
  into `a`).
- `l * k` and `k * l` multiply the value of the length `l` by the number `k` converted into an
  `f64`, and the result is a length of the same unit. With `k` on the right, `k` can be of any
  number type that converts into an `f64` with `Into`, such as `u8` or `i16`. With `k` on the
  left, the tests use `f64`, `f32`, `i32` and `u32`.
- `perimeter(w, h)` returns the perimeter of a `w` × `h` rectangle, `2 * (w + h)`.
- `to_mm(l)` converts inches to millimeters: the value is multiplied by `25.4`.
- All of this must hold for **every** unit type `U`. Besides `Mm` and `Inch`, the tests define
  units of their own, such as `struct Cubit;` and `enum Furlongs {}`, which derive the same four
  traits. They also use lengths in generic code that knows nothing about `U`.

Do not change the types or the signatures of `new`, `value`, `perimeter` and `to_mm`. Anything else
may be changed as long as the rules above hold.

### Example

```rust
let a = Length::<Mm>::new(2.0);
a + a                           // Length::new(4.0)
a * 3                           // Length::new(6.0)
3 * a                           // Length::new(6.0)
perimeter(a, Length::new(3.0))  // Length::new(10.0)
a.value()                       // 2.0 (`a` can still be used)

let l = Length::<Inch>::new(2.0);
2.5 * l                                    // Length::new(5.0)
to_mm(l).value()                           // 50.8
Length::<Mm>::new(6.0) / Length::new(1.5)  // 4.0
```
