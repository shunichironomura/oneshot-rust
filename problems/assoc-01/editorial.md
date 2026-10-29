An associated type belongs to the impl: every container type has exactly one `Item`, and a generic
function names it `C::Item` (short for `<C as Container>::Item`) once it has the bound
`C: Container`.

`move_all` needs **two** type parameters. With a single `C` for both arguments, `from` and `to` must
be the same type, so moving from `Stack<u8>` to `Bag` is rejected. With two unrelated parameters
`A` and `B`, the body fails instead: `to.put(x)` expects a `B::Item` but gets an `A::Item`. The
equality constraint `B: Container<Item = A::Item>` says that `B` holds the same type of values as
`A`. (Rust has no `where A::Item == B::Item` clause.) No other bound is needed: the values are only
moved, which is why `Book`, which is not `Clone`, works.

### Alternatives

`move_all` can also name the shared item type with a third type parameter.

```rust
pub fn move_all<T, A, B>(from: &mut A, to: &mut B) -> usize
where
    A: Container<Item = T>,
    B: Container<Item = T>,
{
    let mut n = 0;
    while let Some(x) = from.take_last() {
        to.put(x);
        n += 1;
    }
    n
}
```

With `impl Trait` in argument position, the container types have no names, so the item type needs
one: `T`.

```rust
pub fn move_all<T>(from: &mut impl Container<Item = T>, to: &mut impl Container<Item = T>) -> usize {
    let mut n = 0;
    while let Some(x) = from.take_last() {
        to.put(x);
        n += 1;
    }
    n
}
```

Trait objects work as well. A `dyn Container` must name its associated type, so the function is
generic only over the item type `T`, and `&mut Bag` coerces to `&mut dyn Container<Item = u8>`. The
methods are then called through dynamic dispatch.

```rust
pub fn move_all<T>(from: &mut dyn Container<Item = T>, to: &mut dyn Container<Item = T>) -> usize {
    let mut n = 0;
    while let Some(x) = from.take_last() {
        to.put(x);
        n += 1;
    }
    n
}
```
