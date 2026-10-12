Compare two lists of words as if they were sets: which words they share, which words only one of
them has, and so on.

```rust
#[derive(Debug, PartialEq)]
pub struct Overlap {
    pub both: Vec<String>,
    pub only_left: Vec<String>,
    pub only_right: Vec<String>,
    pub exactly_one: Vec<String>,
    pub all: Vec<String>,
}

pub fn overlap(left: &[&str], right: &[&str]) -> Overlap
```

The struct `Overlap` is already in the editor.

- Words are compared exactly as they are: `"A"` and `"a"` are different words.
- `overlap` only looks at whether a word appears in a list, not at how many times. The fields of
  the returned `Overlap` are:
  - `both`: the words that are in both `left` and `right`.
  - `only_left`: the words that are in `left` but not in `right`.
  - `only_right`: the words that are in `right` but not in `left`.
  - `exactly_one`: the words that are in exactly one of the two lists.
  - `all`: the words that are in at least one of the two lists.
- In each of the five vectors, every word appears at most once, and the words are sorted in
  ascending order, comparing strings with `<`. This compares bytes, so uppercase letters come before
  lowercase ones (`"Z" < "a"`), a word comes before longer words that start with it
  (`"app" < "apple"`), and `"z" < "é"`.
- Either list may be empty.

### Examples

```rust
overlap(&["b", "a", "c", "a"], &["d", "c", "b"])
// Overlap {
//     both: ["b", "c"],
//     only_left: ["a"],
//     only_right: ["d"],
//     exactly_one: ["a", "d"],
//     all: ["a", "b", "c", "d"],
// }
```
