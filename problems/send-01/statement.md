The function in the editor maps a closure over a list of items, running each call on its own
thread. Its body is already written, but its signature is wrong, so the code does not compile.
Change **only the signature** (the `pub fn` line and its `where` clause) so that the test harness
(the `tests.rs` tab) compiles and passes. Do not change the body.

- `par_map(items, f)` calls `f` exactly once for every item, each call on its own scoped thread, and
  returns the results in the order of `items`. If `items` is empty, it returns an empty `Vec`.

The harness calls it with many different closures and items. Every one of these calls has to
compile.

### Examples

```rust
let calls = AtomicUsize::new(0);
let prefix = String::from("item-");
let names = vec![String::from("a"), String::from("bb"), String::from("ccc")];
par_map(names, |name| {
    calls.fetch_add(1, Ordering::Relaxed);
    format!("{prefix}{name}")
})
// ["item-a", "item-bb", "item-ccc"], and calls is now 3
```
