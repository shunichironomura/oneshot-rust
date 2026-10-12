The bodies of the four functions below are already written, but each signature is tied to a single
concrete type, so the test harness (the `tests.rs` tab) does not compile. Change **only the
signatures** so that the harness compiles and passes. Do not change the bodies.

```rust
pub fn char_len(s: &str) -> usize
pub fn join_upper(words: &[String], sep: &str) -> String
pub fn total_chars(groups: Vec<Vec<String>>) -> usize
pub fn square_in_place(n: &mut u32)
```

- `char_len` returns the number of characters (`char`s) in a string. The harness passes `&str`,
  `String` and `&String` values.
- `join_upper` converts every word to upper case and joins them, with `sep` between neighbors. The
  harness passes references to arrays of `&str` and to `Vec<String>`s, and a slice of a
  `Vec<String>`. For an empty list it names the element type, as in `join_upper::<String>(&[], ",")`.
- `total_chars` returns the total number of characters (`char`s) in groups of strings (`&str` or
  `String`). The harness passes `Vec`s of `Vec`s, arrays of arrays or of `Vec`s, references to
  these, and iterators whose items are iterators over `&str` (such as
  `text.lines().map(|l| l.split(' '))`).
- `square_in_place` replaces a `u32` with its square. The harness passes a `&mut Box<u32>` and a
  mutable reference to its own type `Score`, which implements `AsMut<u32>`.

### Examples

```rust
char_len("Café")                           // 4
char_len(String::from("ab"))               // 2
join_upper(&["ab", "c"], "-")              // "AB-C"
join_upper(&vec![String::from("é")], ",")  // "É"
total_chars([["a", "b"]; 3])               // 6
total_chars(&[vec![String::from("é")]])    // 1

let mut b = Box::new(3u32);
square_in_place(&mut b);                   // *b is now 9
```
