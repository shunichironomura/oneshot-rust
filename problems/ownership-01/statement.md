The code below does not compile. Fix it.

`greet_all` takes a list of names, builds the greeting `"Hello, <name>!"` for each of them, and also
returns how many people it greeted.

```rust
pub fn greet_all(names: Vec<String>) -> (Vec<String>, usize)
```

Do not change the signature.

### Example

`greet_all(vec!["Ferris".into()])` returns `(vec!["Hello, Ferris!"], 1)`.
