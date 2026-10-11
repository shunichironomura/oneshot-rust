`BTreeSet` does most of the work. Collecting a list into a set drops the duplicates, and a
`BTreeSet` always iterates in ascending order, so `intersection`, `difference`,
`symmetric_difference` and `union` yield the words already sorted and without repeats. Strings are
ordered by their bytes, which is exactly the order the statement asks for.

Two type details often break a first attempt:

- `left.iter()` yields `&&str` (a reference to each element of the slice), so
  `left.iter().collect::<BTreeSet<&str>>()` does not compile. `.copied()` turns the items into
  `&str`.
- The set operations also yield references to the elements, `&&str` here. Collecting them into a
  `Vec<String>` needs a conversion such as `.map(|w| w.to_string())`; `.cloned()` only gets you
  back to `&str`.

A `HashSet` works too, but its iteration order is arbitrary, so every vector must be sorted
afterwards. Without the sort, a small example may still come out in order by luck, which makes the
mistake easy to miss.

`first_repeat` relies on the return value of `insert`: `true` when the value is new, `false` when
it is already in the set. The first word whose `insert` returns `false` is the answer.
`for &word in words` destructures each `&&'a str` into a `&'a str`, so the word can be returned as
it is: it borrows from the original strings, not from `words` or from the set.

### Alternatives

With a `HashSet`, sort each vector at the end.

```rust
use std::collections::HashSet;

pub fn overlap(left: &[&str], right: &[&str]) -> Overlap {
    let left: HashSet<&str> = left.iter().copied().collect();
    let right: HashSet<&str> = right.iter().copied().collect();
    Overlap {
        both: sorted(left.intersection(&right)),
        only_left: sorted(left.difference(&right)),
        only_right: sorted(right.difference(&left)),
        exactly_one: sorted(left.symmetric_difference(&right)),
        all: sorted(left.union(&right)),
    }
}

fn sorted<'a>(words: impl Iterator<Item = &'a &'a str>) -> Vec<String> {
    let mut words: Vec<String> = words.map(|w| w.to_string()).collect();
    words.sort();
    words
}
```

The operators `&`, `-`, `^` and `|` on two `&BTreeSet`s compute the same sets and return new
`BTreeSet`s (`BTreeSet<&str>` here), which are easy to turn into vectors.

```rust
use std::collections::BTreeSet;

pub fn overlap(left: &[&str], right: &[&str]) -> Overlap {
    let left: BTreeSet<&str> = left.iter().copied().collect();
    let right: BTreeSet<&str> = right.iter().copied().collect();
    Overlap {
        both: owned(&left & &right),
        only_left: owned(&left - &right),
        only_right: owned(&right - &left),
        exactly_one: owned(&left ^ &right),
        all: owned(&left | &right),
    }
}

fn owned(words: BTreeSet<&str>) -> Vec<String> {
    words.into_iter().map(String::from).collect()
}
```

`first_repeat` fits in one expression with `Iterator::find`. The closure may modify `seen`, because
`find` accepts an `FnMut` closure.

```rust
pub fn first_repeat<'a>(words: &[&'a str]) -> Option<&'a str> {
    let mut seen = HashSet::new();
    words.iter().copied().find(|&word| !seen.insert(word))
}
```
