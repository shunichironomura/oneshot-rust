An associated type belongs to the impl: every container type has exactly one `Item`, and a generic
function names it `C::Item` (short for `<C as Container>::Item`) once it has the bound
`C: Container`. So `total` becomes generic over the container, and the bounds that the body needs go
on `C::Item`, the values, not on `C`: the container itself is never converted.

The body needs two things from `C::Item`.

- The closure pattern `|&x|` copies the value out of the `&C::Item` that the iterator yields, so the
  item must be `Copy`. `Clone` is not enough ("cannot move out of a shared reference").
- `x.into()` never says what it converts into. The bound `C::Item: Into<i64>` answers that: when a
  where clause matches the obligation `C::Item: Into<_>`, the compiler uses it and infers `i64`. The
  bound `i64: From<C::Item>` also makes the conversion possible (through the blanket impl of
  `Into`), but it leaves the target of `into` open, and `sum()` cannot settle it either, because
  `i64` implements both `Sum<i64>` and `Sum<&i64>`. The result is E0283 "type annotations needed".
  The same error is why the template's body does not compile even for `Bag`: a `u8` converts into
  many types.

### Alternatives

With `impl Trait` in argument position, the bounds on the item can be written as associated type
bounds (`Item: Bound`).

```rust
pub fn total(c: &impl Container<Item: Copy + Into<i64>>) -> i64 {
    c.items().iter().map(|&x| x.into()).sum()
}
```

A trait object works as well. A `dyn Container` must name its associated type, so the function is
generic only over the item type `T`, and `&Bag` coerces to `&dyn Container<Item = u8>`.

```rust
pub fn total<T: Copy + Into<i64>>(c: &dyn Container<Item = T>) -> i64 {
    c.items().iter().map(|&x| x.into()).sum()
}
```
