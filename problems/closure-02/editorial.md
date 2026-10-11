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
- `each_digit` uses `f` twice: it passes `f` on to the recursive call for the leading digits, then
  calls it for the last digit. Taken by value, `f` is moved into the recursive call, and the call
  after it fails with "borrow of moved value: `f`" (E0382). The harness passes `&mut` references to
  its closures, so take one: `f: &mut impl FnMut(u64)`. A `&mut F` can be called like `F` itself
  (no `mut f` needed), and passing `f` where a `&mut` parameter is expected reborrows it (as
  `&mut *f`) instead of moving it, so `f` is still there afterwards. `FnMut` is again the right
  bound, because the closures push to a `Vec` or add to a sum. The compiler's hint
  `each_digit(n / 10, &mut f)` would change the body, and it does not work in a generic function
  anyway: each level of recursion would call `each_digit` with a new type `&mut &mut ... F`, and
  rustc gives up with "reached the recursion limit while instantiating".

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

pub fn each_digit<F>(n: u64, f: &mut F)
where
    F: FnMut(u64),
{
    if n >= 10 {
        each_digit(n / 10, f);
    }
    f(n % 10);
}
```

`each_digit` can also take a trait object. There is then a single compiled copy of the function for
all closures, and every call of `f` goes through the vtable.

```rust
pub fn each_digit(n: u64, f: &mut dyn FnMut(u64)) {
    if n >= 10 {
        each_digit(n / 10, f);
    }
    f(n % 10);
}
```
