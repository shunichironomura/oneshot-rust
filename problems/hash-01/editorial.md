`HashSet` and `HashMap` find a key in two steps: the hash decides where to look, and `==` checks the
entries found there. This only works if **equal keys have equal hashes**, which is the contract of
`Hash`: `a == b` must imply `hash(a) == hash(b)`. Once `eq` ignores case, `hash` must ignore it too.

The tempting `self.0.hash(state)` (exactly what `#[derive(Hash)]` would generate) hashes `Rust` and
`RUST` differently. `contains` then usually looks in the wrong place and returns `false`, and
`unique_tags` keeps both spellings. This is a logic error, not UB: nothing crashes, the results are
just wrong, and a lucky collision can even make a lookup succeed now and then.

So hash exactly what `eq` compares: the bytes with ASCII letters lowercased. `eq_ignore_ascii_case`
and `to_ascii_lowercase` only change `A`–`Z`, as the rules ask. `to_lowercase` and `to_uppercase`
follow Unicode instead: comparing with them makes `É` equal to `é`, and `to_uppercase` even makes
`Straße` equal to `STRASSE`. The final `0xff` byte copies what `str` does: it marks the end of the
string, so that the pairs `("ab", "c")` and `("a", "bc")` hash differently (the tests do not need it).
Unequal tags may still get the same hash: that is only a collision.

Keeping the first spelling needs no extra code. `HashSet::insert` returns `false` and leaves the
stored value alone when an equal one is already there, and `HashMap::entry` (like `HashMap::insert`)
keeps the existing key and drops the new one. That is also why `set.get(&Tag::new("rust"))` returns
the stored `Rust`. `Display` simply writes that stored spelling, with `f.write_str(&self.0)` or
`write!(f, "{}", self.0)`.

The documentation of `Borrow` has a similar `CaseInsensitiveString` example and explains why such a
type must not implement `Borrow<str>`: `str` compares and hashes with case, so a lookup with
`map.get("rust")` would disagree with the keys.

### Alternatives

Hashing a lowercased copy is the shortest correct `hash`, at the cost of an allocation per call.

```rust
impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_ascii_lowercase().hash(state);
    }
}
```

`eq` could compare lowercased copies the same way, but `eq_ignore_ascii_case` does it without
allocating.

`unique_tags` can also avoid hashing by checking the tags collected so far with `==`. This takes
quadratic time, so the `HashSet` version is better for long lists.

```rust
pub fn unique_tags(words: &[&str]) -> Vec<Tag> {
    let mut tags: Vec<Tag> = Vec::new();
    for &word in words {
        let tag = Tag::new(word);
        if !tags.contains(&tag) {
            tags.push(tag);
        }
    }
    tags
}
```
