A *bag* (a multiset) is like a set that also remembers how many times each element was added.
Make `Bag<T>` work with Rust's iterator machinery: `collect`, `extend`, `for` loops and `rev`.

```rust
pub struct Bag<T> {
    counts: BTreeMap<T, usize>,
}

impl<T: Ord> Bag<T> {
    pub fn new() -> Self;
    pub fn count(&self, x: &T) -> usize;
    pub fn len(&self) -> usize;
}

impl<T: Ord> FromIterator<T> for Bag<T>
impl<T: Ord> Extend<T> for Bag<T>
impl<'a, T: Ord + Copy + 'a> Extend<&'a T> for Bag<T>

// for (x, n) in &bag
impl<'a, T> IntoIterator for &'a Bag<T> // Item = (&'a T, usize), IntoIter = Iter<'a, T>
// for x in bag
impl<T: Clone> IntoIterator for Bag<T> // Item = T, IntoIter = IntoIter<T>
```

All of this is already in the editor with `todo!()` bodies, together with the iterator types `Iter`
and `IntoIter` and their `Iterator` and `DoubleEndedIterator` impls. The fields of `Iter` and
`IntoIter` are placeholders: replace them with whatever you need (you may also add bounds to these
two structs). Do not change the other signatures.

- `new` returns an empty bag. `count(&x)` is how many times an element equal to `x` has been added
  (`0` if none has). `len` is the total number of elements, duplicates included.
- `collect` and `extend` add every element they receive. Like a `Vec`, a bag can also be extended
  with references to `Copy` elements: `bag.extend(&[1, 2, 2])`.
- Iterating a borrowed bag yields one `(element, count)` pair per distinct element, in ascending
  order of the elements. The bag can still be used afterwards.
- Iterating an owned bag yields every element as many times as its count, in ascending order. The
  bag holds only one value for each distinct element, so the other copies have to be clones, but
  no more clones than needed are made:
  - calling `into_iter()` clones nothing;
  - by the time the iterator is used up, an element with count `n` has been cloned exactly `n - 1`
    times (the bag's own value is handed out as one of the `n` copies).
- Both iterators are double-ended: `next_back` takes items from the end (in descending order), and
  calls to `next` and `next_back` can be mixed. Every item is yielded exactly once, whichever end it
  comes from, and once all of them have been yielded, both methods return `None`.

### Examples

```rust
let mut b: Bag<char> = "hello".chars().collect();
// b.count(&'l') == 2, b.len() == 5
b.extend("lo".chars());
b.extend(&['x', 'x']);
for (c, n) in &b {
    // (&'e', 1), (&'h', 1), (&'l', 3), (&'o', 2), (&'x', 2), in this order
}
b.into_iter().collect::<String>() // "ehlllooxx"
```

```rust
let b: Bag<i32> = (1..=3).chain(2..=3).collect();
// b.count(&3) == 2, b.len() == 5
(&b).into_iter().next_back() // Some((&3, 2))
b.into_iter().rev().collect::<Vec<_>>() // [3, 3, 2, 2, 1]
Bag::<i32>::new().into_iter().next() // None
```
