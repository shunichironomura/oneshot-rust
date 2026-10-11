The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

The test harness runs the three async functions below with its own small executor (only `std` is
available, there is no async runtime).

```rust
pub fn greet(name: String) -> impl Future<Output = String>;
pub async fn sum_all(futs: Vec<Pin<Box<dyn Future<Output = u32>>>>) -> u32;

pub struct Tree {
    pub children: Vec<Tree>,
}

pub async fn depth(t: &Tree) -> usize;
```

- `greet(name)` returns a future that resolves to `Hello, {name}!`, with `{name}` replaced by
  `name`. `name` can be any string, including the empty string.
- `sum_all(futs)` awaits the futures one at a time, in the order of the vector: each future finishes
  before the next one is polled for the first time. It resolves to the sum of their outputs, or `0`
  for an empty vector. The sum always fits in a `u32`. Some of the futures the harness passes are
  not ready the first time they are polled (they return `Poll::Pending` once).
- `depth(t)` resolves to the number of levels of the tree: `1` for a node without children,
  otherwise 1 plus the largest depth among its children.

Do not change the signatures or the `Tree` struct. Keep the recursion in `depth` asynchronous: the
depth of each child must still come from an awaited recursive call.

### Examples

- `greet("Ann".to_string())` resolves to `"Hello, Ann!"`.
- Given three futures that resolve to `1`, `2` and `3`, `sum_all` resolves to `6`.
- A root with two children, the first of which has one child of its own, has depth `3`.
