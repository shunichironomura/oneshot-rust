`make_greeter` has two problems, and the compiler reports both at once.

- The returned closure is used after `make_greeter` returns, so it must not borrow anything the
  function owns, its parameters included (E0373, "closure may outlive the current function"): it
  needs `move`.
- But `move` alone is not enough. After `let greeting = greeting.trim();`, the name `greeting`
  refers to a `&str` that borrows the `String` parameter, which is dropped when the function
  returns. Moving that reference into the closure still leaves the closure borrowing the `String`
  (E0597, "`greeting` does not live long enough"). Make the trimmed text an owned `String` with
  `to_string()` and move that.

### Alternatives

The greeter can also move the whole `String` and trim it on every call.

```rust
pub fn make_greeter(greeting: String) -> impl Fn(&str) -> String {
    move |name| format!("{}, {name}!", greeting.trim())
}
```
