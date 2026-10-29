An `async` block captures variables like a closure does, by reference when that is enough.
Formatting only needs `&name`, so the returned future borrows `name`, a local that is dropped when
`greet` returns (E0373, "async block may outlive the current function"). `async move`
moves `name` into the future instead.

- An `async fn` never has this problem, because it always moves its parameters into its future.
  This is why `greet` would compile as `pub async fn greet(name: String) -> String`; the signature
  must stay as it is here, though.
- Writing `name.clone()` inside the block does not help: the clone runs only when the future is
  polled, so the block still borrows `name`.

### Alternatives

`greet` can format the string first and return a future that is already finished. The output is the
same, but the formatting now happens when `greet` is called rather than on the first poll.

```rust
pub fn greet(name: String) -> impl Future<Output = String> {
    std::future::ready(format!("Hello, {name}!"))
}
```
