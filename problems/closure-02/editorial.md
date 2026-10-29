The three closure traits form a ladder. Every closure implements `FnOnce` (it can be called once).
Closures that do not move a captured value out also implement `FnMut`, and those that do not mutate
their captures either also implement `Fn`. A bound cuts both ways: the weaker it is, the more
closures callers may pass, but the less the body may do with `f`. So ask for as little as the body
needs.

- The body calls `f` twice, which rules out `FnOnce`: the first call consumes `f`, and the second
  fails with "use of moved value: `f`" (E0382).
- The numbering closure in the harness updates `next`, which rules out `Fn`. That leaves `FnMut`.
- Calling an `FnMut` needs a mutable borrow of `f`, so the parameter must be `mut f`. Without it,
  the body fails with "cannot borrow `f` as mutable" (E0596).
- The input and output types differ (`&str` to `String`, `i32` to `bool`, `Vec<u8>` to `usize`), so
  the function needs two type parameters, `A` and `B`. Both elements of the pair go through the same
  `f`, so they share the type `A`.

`map_pair(&mut record, ...)` works because a `&mut F` is itself an `FnMut` when `F` is. The closure
is only borrowed, so it can be passed again, and `seen` can be read once its last use is over.

### Alternatives

`impl Trait` in argument position is shorthand for a type parameter. A named type parameter with
the bound in a `where` clause means the same here.

```rust
pub fn map_pair<A, B, F>(mut f: F, pair: (A, A)) -> (B, B)
where
    F: FnMut(A) -> B,
{
    (f(pair.0), f(pair.1))
}
```
