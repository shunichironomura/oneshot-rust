The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

`Length<U>` is a length whose unit is part of its type: `Length<Mm>` and `Length<Inch>` are
different types, so millimeters and inches cannot be mixed up by mistake. A unit type holds no data;
a `Length<U>` stores only an `f64` and a zero-sized `PhantomData<U>`.

```rust
pub struct Mm;
pub struct Inch;

pub struct Length<U> {
    value: f64,
    unit: PhantomData<U>,
}

impl<U> Length<U> {
    pub fn new(value: f64) -> Self;
    pub fn value(self) -> f64;
}

impl<U> Add for Length<U> {
    type Output = Length<U>;
    // ...
}

pub fn perimeter<U>(w: Length<U>, h: Length<U>) -> Length<U>
pub fn longest<U>(items: &[Length<U>]) -> Option<Length<U>>
pub fn to_mm(l: Length<Inch>) -> Length<Mm>
```

- `new` creates a length with the given value, and `value` returns the value.
- `a + b` adds the values. The result has the same unit as `a` and `b`.
- `perimeter(w, h)` returns the perimeter of a `w` × `h` rectangle, `w + w + h + h`.
- `longest(items)` returns a copy of the element with the largest value, or `None` if `items` is
  empty. The slice itself is not changed. If several elements share the largest value, any of them
  may be returned. Values are never NaN.
- `to_mm(l)` converts inches to millimeters: the value is multiplied by `25.4`.
- Lengths behave like plain numbers:
  - `Length<U>` implements `Copy` and `Clone`. The tests keep using a length after passing it by
    value.
  - Two lengths of the same unit are equal (`==`) when their values are equal as `f64`s.
  - `{:?}` prints `Length(<v>)`, where `<v>` is the value printed with `{:?}`, for example
    `Length(2.5)` or `Length(10.0)`.
- All of this must hold for **every** unit type `U`. Besides `Mm` and `Inch`, the tests define units
  of their own that derive nothing and implement no traits at all, such as `struct Cubit;` and
  `enum Furlongs {}`. They also use lengths in generic code that knows nothing about `U`.

Do not change the public signatures. Anything else may be changed as long as the rules above hold.

### Example

```rust
let a = Length::<Mm>::new(2.0);
let p = perimeter(a, Length::new(3.0));
p == Length::new(10.0)  // true
a.value()               // 2.0 (`a` can still be used)
format!("{p:?}")        // "Length(10.0)"

to_mm(Length::<Inch>::new(2.0)).value()  // 50.8
longest(&[Length::<Inch>::new(1.5), Length::new(4.0), Length::new(2.0)])  // Some(Length(4.0))
```
