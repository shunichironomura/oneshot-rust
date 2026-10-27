The code below does not compile. Fix it.

Five small functions answer questions about slices.

```rust
pub fn first_negative(v: &[i32]) -> Option<i32>
pub fn position_of_max(v: &[i32]) -> Option<usize>
pub fn count_long(words: &[&str], min: usize) -> usize
pub fn contains_word(words: &[String], w: &str) -> bool
pub fn total_len(words: &[String]) -> usize
```

- `first_negative` returns the first element that is less than `0`, or `None` if there is none.
- `position_of_max` returns the index of the largest element. If the largest value appears more than
  once, it returns the index of its first occurrence. For an empty slice it returns `None`.
- `count_long` returns how many words are at least `min` bytes long.
- `contains_word` returns whether some element of `words` is exactly equal to `w`. The comparison is
  case-sensitive, and a word that only contains `w` as a part does not count.
- `total_len` returns the sum of the lengths of all words, in bytes.

Lengths are byte lengths as returned by `len()`, not numbers of characters: `"é"` is 2 bytes long.
Do not change the signatures.

### Examples

- `first_negative(&[3, -1, -5])` returns `Some(-1)`.
- `position_of_max(&[1, 9, 9])` returns `Some(1)`.
- `count_long(&["a", "abc", "de"], 2)` returns `2`.
- With `words = ["apple", "pie"]` (as `String`s), `contains_word(&words, "pie")` returns `true`.
- With `words = ["ab", "cde"]` (as `String`s), `total_len(&words)` returns `5`.
