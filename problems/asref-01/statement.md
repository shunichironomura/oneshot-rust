The body of the function below is already written, but its signature is tied to a single concrete
element type, so the test harness (the `tests.rs` tab) does not compile. Change **only the
signature** so that the harness compiles and passes. Do not change the body.

```rust
pub fn join_upper(words: &[String], sep: &str) -> String
```

- `join_upper` converts every word to upper case and joins them, with `sep` between neighbors. The
  harness passes references to arrays of `&str` and to `Vec<String>`s, and a slice of a
  `Vec<String>`. For an empty list it names the element type, as in `join_upper::<String>(&[], ",")`.

### Examples

```rust
join_upper(&["ab", "c"], "-")              // "AB-C"
join_upper(&vec![String::from("é")], ",")  // "É"
join_upper::<String>(&[], ",")             // ""
```
