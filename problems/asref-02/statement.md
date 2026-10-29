The function below counts the characters in groups of strings. Its body is already written, but its
signature accepts only a `Vec` of `Vec<String>`s, so the test harness (the `tests.rs` tab) does not
compile. Change **only the signature** (plus a `where` clause if you want one) so that the harness
compiles and passes. Do not change the body.

```rust
pub fn total_chars(groups: Vec<Vec<String>>) -> usize
```

- `total_chars` returns the total number of characters (`char`s) in groups of strings (`&str` or
  `String`). The harness passes `Vec`s of `Vec`s, arrays of arrays or of `Vec`s, references to
  these, and iterators whose items are iterators over `&str` (such as
  `text.lines().map(|l| l.split(' '))`).

### Examples

```rust
total_chars(vec![vec!["ab", "c"], vec![]])          // 3
total_chars([["a", "b"]; 3])                        // 6
total_chars(&[vec![String::from("é")]])             // 1
total_chars("a b\nc".lines().map(|l| l.split(' ')))  // 3
```
