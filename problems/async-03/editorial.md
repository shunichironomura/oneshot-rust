In a trait, `async fn get(&self, key: &str) -> Option<u32>` is shorthand for
`fn get(&self, key: &str) -> impl Future<Output = Option<u32>>`. Every implementation returns a
future type of its own, and the trait promises nothing about it except its output; in particular,
not `Send`. In `total_on_worker`, all the compiler knows about `S` is `S: Store`, so it cannot
assume that the future of `S::get`, which the future of `total` keeps while awaiting it, may move to
another thread. With a concrete type such as `MemStore`, the compiler can see the actual future and
would accept the code, which is why the problem only appears in generic code.

The fix is to write the method in its desugared form and add the bound:
`fn get(&self, key: &str) -> impl Future<Output = Option<u32>> + Send;`. `Send` is now part of the
trait's contract: generic code may rely on it, and every implementation is checked against it. An
implementation may still be an `async fn`, as long as its future is `Send`, so `MemStore`,
`Fallback` and the harness's store stay as they are. (In the 2024 edition, `Future` is in the
prelude.)

That is not enough on its own. A future is `Send` only if everything it holds across an `.await` is,
and the future of `total` holds `s: &S`. A shared reference `&S` is `Send` only when `S: Sync`. In
fact, this is the only error the compiler reports for the template: the one about the future of
`S::get` appears only once it is fixed. `Fallback` has the same problem: its `get` holds `&self`, a
`&Fallback<A, B>`, while it awaits `first`, so once the trait requires `Send`, its implementation
needs `A: Sync` and `B: Sync`. Neither the harness nor the implementations may change, so the trait
must provide it. `pub trait Store: Sync` makes `Sync` a supertrait: every type that implements
`Store` must be `Sync`, and generic code may assume it.

These attempts fail:

- Only one of the two changes: the other error remains.
- `where Self: Sync` on `get` alone: callers such as `total` and `Fallback` would have to prove
  `S: Sync` (or `A: Sync`) themselves.
- `+ Send + '_` or `+ Send + 'static`: the future of an `async fn` borrows both `self` and `key`, so
  it satisfies neither bound (E0477). No lifetime bound is needed: an `impl Trait` return type in a
  trait may capture all the lifetimes of the method.
- Returning `Pin<Box<dyn Future<Output = Option<u32>> + Send + '_>>`: the implementations written as
  `async fn` no longer match the trait (E0053).

The lint `async_fn_in_trait` warns about `async fn` in public traits for exactly this reason. It is
silent here because the trait is not visible outside the crate.

### Alternatives

`Send` can be a supertrait as well, and the lifetimes can be written out.

```rust
pub trait Store: Send + Sync {
    fn get<'a>(&'a self, key: &'a str) -> impl Future<Output = Option<u32>> + Send + 'a;
}
```

A bound on `Self` in a `where` clause of the trait is also a supertrait.

```rust
pub trait Store
where
    Self: Sync,
{
    fn get(&self, key: &str) -> impl Future<Output = Option<u32>> + Send;
}
```
