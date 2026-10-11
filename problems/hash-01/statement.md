Tags on a blog ignore case: `Rust`, `rust` and `RUST` are the same tag, but a tag remembers how it
was written. Implement the type `Tag` (it is already in the editor) and two functions that use it.

```rust
#[derive(Debug, Clone)]
pub struct Tag(String);

impl Tag {
    pub fn new(s: &str) -> Tag;
    pub fn as_str(&self) -> &str;
}

impl PartialEq for Tag { /* to implement */ }
impl Eq for Tag {}
impl Hash for Tag { /* to implement */ }
impl fmt::Display for Tag { /* to implement */ }

pub fn unique_tags(words: &[&str]) -> Vec<Tag>
pub fn tag_counts(words: &[&str]) -> HashMap<Tag, usize>
```

- `Tag::new(s)` makes a tag spelled exactly `s`, without trimming (`s` may be empty). `as_str`
  returns that spelling, and `Display` prints it as is.
- Two tags are equal when their spellings are equal **ignoring ASCII case**: `A`–`Z` match `a`–`z`.
  Every other character, non-ASCII letters included, must match exactly. So `Café` equals `CAFé`
  but not `CAFÉ`, `É` and `é` are different tags, and so are `Straße` and `STRASSE`.
- `Tag` must work correctly as a key of `HashSet` and `HashMap`. Besides calling the two functions,
  the tests compare tags with `==` and `!=`, put them in sets and maps, look them up with a
  differently cased spelling, and hash them with `DefaultHasher`.
- `unique_tags` returns one tag for each distinct tag in `words`, in the order of their first
  appearance. Each tag is spelled as it first appeared.
- `tag_counts` counts how many times each tag appears in `words`. Each key is spelled as the tag
  first appeared.
- `words` may be empty.

### Examples

```rust
unique_tags(&["Rust", "go", "RUST", "Go"])
// the tags Rust and go, in this order

tag_counts(&["Rust", "go", "rust", "RUST"])
// {Rust: 3, go: 1}

let mut set = HashSet::new();
set.insert(Tag::new("Rust"))                  // true
set.contains(&Tag::new("rUST"))               // true
set.insert(Tag::new("RUST"))                  // false: an equal tag is already there
set.get(&Tag::new("rust")).unwrap().as_str()  // "Rust"
```
