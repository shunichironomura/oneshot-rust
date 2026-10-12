Find out whether a list of words repeats itself: the first word that appears a second time, and
whether every word appears only once.

```rust
pub fn first_repeat<'a>(words: &[&'a str]) -> Option<&'a str>
pub fn all_unique(words: &[&str]) -> bool
```

- Words are compared exactly as they are: `"A"` and `"a"` are different words.
- `first_repeat` reads `words` from the start and returns the first word it meets that already
  appeared earlier in the list. It returns `None` if no word appears more than once.
- `all_unique` returns `true` if no word appears more than once in `words`, and `false` otherwise.
  An empty list is all unique.

### Examples

```rust
first_repeat(&["a", "b", "c", "b", "a"])
// Some("b"): the second "b" comes before the second "a"
first_repeat(&["a", "b", "c"])  // None

all_unique(&["a", "b", "c"])    // true
all_unique(&["a", "b", "a"])    // false
```
