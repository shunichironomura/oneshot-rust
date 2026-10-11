An associated type belongs to the impl: every container type has exactly one `Item`, and a generic
function names it `C::Item` (short for `<C as Container>::Item`) once it has the bound
`C: Container`. So each function becomes generic over the container, and the bounds that the body
needs go on `C::Item`, the values, not on `C`: the containers themselves are never converted,
compared or displayed.

- `move_all` needs **two** type parameters. With a single `C` for both arguments, `from` and `to`
  must be the same type, so moving from `Stack<u8>` to `Bag` is rejected. With two unrelated
  parameters `A` and `B`, the body fails instead: `to.put(x)` expects a `B::Item` but gets an
  `A::Item`. The equality constraint `B: Container<Item = A::Item>` says that `B` holds the same
  type of values as `A`. (Rust has no `where A::Item == B::Item` clause.) No other bound is needed:
  the values are only moved, which is why `Book`, which is not `Clone`, works.
- `total` needs two things from `C::Item`. The closure pattern `|&x|` copies the value out of the
  `&C::Item` that the iterator yields, so the item must be `Copy`. `Clone` is not enough ("cannot
  move out of a shared reference"). And `x.into()` never says what it converts into. The bound
  `C::Item: Into<i64>` answers that: when a where clause matches the obligation `C::Item: Into<_>`,
  the compiler uses it and infers `i64`. The bound `i64: From<C::Item>` also makes the conversion
  possible (through the blanket impl of `Into`), but it leaves the target of `into` open, and
  `sum()` cannot settle it either, because `i64` implements both `Sum<i64>` and `Sum<&i64>`. The
  result is E0283 "type annotations needed". The same error is why the template's body does not
  compile even for `Bag`: a `u8` converts into many types.
- `largest` needs `C::Item: Ord`, because `Iterator::max` requires a total order. `PartialOrd` is
  not enough: `f64` implements it too, but a `NaN` cannot be compared with any value, so `max`
  could not always decide. The return type `Option<&C::Item>` is a reference into the container.
  `c` is the only reference parameter, so lifetime elision ties the result to `c`, and no lifetime
  has to be written.
- `describe_all` needs `C::Item: Display`. `to_string` comes from `ToString`, which every `Display`
  type implements, so `C::Item: ToString` works too. `Debug` would be wrong: `{}` means `Display`,
  and `Book` does not even implement `Debug`.

### Alternatives

With `impl Trait` in argument position, bounds on the item can be written as associated type bounds
(`Item: Bound`). The container type then has no name, so where the signature has to mention the
item type, a type parameter `T` names it.

```rust
use std::fmt::Display;

pub fn move_all<T>(from: &mut impl Container<Item = T>, to: &mut impl Container<Item = T>) -> usize
pub fn total(c: &impl Container<Item: Copy + Into<i64>>) -> i64
pub fn largest<T: Ord>(c: &impl Container<Item = T>) -> Option<&T>
pub fn describe_all(c: &impl Container<Item: Display>) -> Vec<String>
```

Trait objects work as well. A `dyn Container` must name its associated type, so each function is
generic only over the item type `T`, and `&Bag` coerces to `&dyn Container<Item = u8>`. The methods
are then called through dynamic dispatch.

```rust
pub fn move_all<T>(from: &mut dyn Container<Item = T>, to: &mut dyn Container<Item = T>) -> usize
pub fn total<T: Copy + Into<i64>>(c: &dyn Container<Item = T>) -> i64
pub fn largest<T: Ord>(c: &dyn Container<Item = T>) -> Option<&T>
pub fn describe_all<T: Display>(c: &dyn Container<Item = T>) -> Vec<String>
```

`move_all` can also name the shared item type with a third type parameter.

```rust
pub fn move_all<T, A, B>(from: &mut A, to: &mut B) -> usize
where
    A: Container<Item = T>,
    B: Container<Item = T>,
```
