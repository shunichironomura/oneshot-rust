The code below does not compile. Fix it.

`make_greeter` builds a function that greets people.

```rust
pub fn make_greeter(greeting: String) -> impl Fn(&str) -> String
```

- `make_greeter(greeting)` returns a function that turns `name` into `"<greeting>, <name>!"`, where
  `<greeting>` is `greeting` with the whitespace at both ends removed (as `str::trim` does).
- `name` is used as is.
- The returned function can be called any number of times.

Do not change the signature. Anything inside the function body may be changed.

### Examples

```rust
let hi = make_greeter(String::from("  Hi "));
hi("Ann")  // "Hi, Ann!"
```
