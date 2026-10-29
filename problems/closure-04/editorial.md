`each_digit` uses `f` twice: it passes `f` on to the recursive call for the leading digits, then
calls it for the last digit. Taken by value, `f` is moved into the recursive call, and the call
after it fails with "borrow of moved value: `f`" (E0382). Changing only `Fn` to `FnMut` in the
template does not help, even though the harness's `&mut` closures fit that bound: the move is still
there.

The harness passes `&mut` references to its closures, so take one: `f: &mut impl FnMut(u64)`.

- A `&mut F` can be called like `F` itself, so no `mut f` is needed.
- Passing `f` where a `&mut` parameter is expected reborrows it (as `&mut *f`) instead of moving it,
  so `f` is still there for `f(n % 10)`. The caller's closure is only borrowed as well, which is why
  the harness can pass `add` three times and read `sum` afterwards.
- The bound is `FnMut`, not `Fn`, because the closures push to a `Vec` or add to a sum.

The compiler's hint `each_digit(n / 10, &mut f)` would change the body, and a normal build of this
generic function rejects it anyway: each level of recursion would call `each_digit` with a new type
`&mut &mut ... F`, and rustc gives up with "reached the recursion limit while instantiating". (The
judge does not catch this: Miri instantiates a function only when a call reaches it, and a `u64`
has at most 20 digits.)

### Alternatives

A named type parameter with the bound in a `where` clause means the same.

```rust
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
