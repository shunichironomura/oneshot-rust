The bodies of the two functions in the editor are already written, but their signatures are wrong,
so the code does not compile. Change **only the signatures** (the `pub fn` lines and their `where`
clauses) so that the test harness (the `tests.rs` tab) compiles and passes. Do not change the
bodies.

- `par_map(items, f)` calls `f` exactly once for every item, each call on its own scoped thread, and
  returns the results in the order of `items`. If `items` is empty, it returns an empty `Vec`.
- `spawn_job(input, f)` starts a new thread that runs `f(input)` and returns its `JoinHandle` right
  away, without waiting for the thread. Joining the handle gives the result of `f(input)`.

The harness calls both functions with many different closures and inputs. Every one of these calls
has to compile.

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

let name = String::from("Ferris");
spawn_job(3, move |n: usize| name + &"!".repeat(n)).join().unwrap()
// "Ferris!!!"
```
