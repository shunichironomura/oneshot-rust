`for name in names` **moves** `names` into the loop, so `names.len()` after the loop uses a moved
value. Iterating over `&names` borrows instead, and `name` becomes a `&String`, which `format!` can
use just as well.

### Alternatives

Build the greetings with an iterator. The borrow in `names.iter()` ends at `collect`, so
`names.len()` can be called afterwards.

```rust
pub fn greet_all(names: Vec<String>) -> (Vec<String>, usize) {
    let greetings = names.iter().map(|name| format!("Hello, {name}!")).collect();
    (greetings, names.len())
}
```

Reading `let count = names.len();` before the loop also works, and then the loop may consume `names`.
