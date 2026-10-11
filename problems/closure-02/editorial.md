The three closure traits form a ladder. Every closure implements `FnOnce` (it can be called once).
Closures that do not move a captured value out also implement `FnMut`, and those that do not mutate
their captures either also implement `Fn`. A bound cuts both ways: the weaker it is, the more
closures callers may pass, but the less the body may do with `f`. So for each function, ask for as
little as its body needs.

- `apply_n` calls `f` again and again, so `FnOnce` is too weak. The harness passes `|| count += 2`,
  which mutates its capture, so `Fn` is too strong. That leaves `FnMut`, and calling an `FnMut` needs
  a mutable borrow of `f`: without `mut f`, the body fails with "cannot borrow `f` as mutable"
  (E0596).
- `consume` calls `f` only once, so `FnOnce` is enough, and the harness needs exactly that:
  `move || name` gives away the `String` it owns, so it can run only once and is not `FnMut` (any
  stronger bound fails with E0507). The result is a `String` in one call and a `Vec` in another,
  hence the type parameter `T`.
- `map_pair` calls `f` twice, which rules out `FnOnce` ("use of moved value: `f`"), and the
  numbering closure in the harness updates `next`, which rules out `Fn`. The input and output types
  differ (`&str` to `String`, `i32` to `bool`), so it needs two type parameters, `A` and `B`.
- `call_shared` was already right. Its body calls `f` through `g = &f`, and a shared reference `&F`
  is callable only when `F: Fn`. With `FnMut`, the body fails with "cannot borrow `*g` as mutable,
  as it is behind a `&` reference". A plain `fn() -> u32` does not work either, because the harness
  passes closures that capture variables, such as `|| k + 1`.
- In `ops_table`, every closure has its own anonymous type, even when two closures look the same, so
  `impl Fn(i32, i32) -> i32`, which stands for one single type, cannot describe all three. Closures
  that capture nothing, however, coerce to the function pointer type `fn(i32, i32) -> i32`, which
  all three share. The return type also tells `|a, b|` that its parameters are `i32`. The compiler's
  hint to box the closures would mean changing the body.

### Alternatives

`impl Trait` in argument position is shorthand for a type parameter. Named type parameters with the
bounds in `where` clauses mean the same here.

```rust
pub fn apply_n<F>(mut f: F, n: usize)
where
    F: FnMut(),
{
    for _ in 0..n {
        f();
    }
}

pub fn consume<T, F>(f: F) -> T
where
    F: FnOnce() -> T,
{
    f()
}

pub fn map_pair<A, B, F>(mut f: F, pair: (A, A)) -> (B, B)
where
    F: FnMut(A) -> B,
{
    (f(pair.0), f(pair.1))
}

pub fn call_shared<F>(f: F) -> u32
where
    F: Fn() -> u32,
{
    let g = &f;
    g() + g()
}
```

`ops_table` has no alternative: without changing the body, `fn(i32, i32) -> i32` is the only return
type that the harness accepts.
