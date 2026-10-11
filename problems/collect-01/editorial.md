The trait impls themselves are short. `FromIterator` starts from an empty bag and hands the work to
`Extend<T>`, which bumps a counter with `*self.counts.entry(x).or_insert(0) += 1`.
`Extend<&'a T>` is the impl `Vec` also has for `Copy` elements: copy each element and reuse
`Extend<T>` with `self.extend(iter.into_iter().copied())`. The compiler picks `Extend<T>` there
because `copied()` yields `T`.

The real work is in the iterator types. `type IntoIter` must be a type you can write down, and an
adapter such as `map` or `flat_map` that holds a closure has a type nobody can name (`impl Trait` is
not allowed for associated types in stable Rust). So `Iter` and `IntoIter` are structs that wrap the
map's own iterators.

- `Iter` wraps `btree_map::Iter<'a, T, usize>`, which yields `(&T, &usize)`; `|(x, &n)| (x, n)`
  turns each item into `(&T, usize)`, and `next_back` comes from the map as well. Write the path
  `btree_map::Iter`: importing `std::collections::btree_map::Iter` would clash with our own `Iter`
  (E0255).
- `IntoIter` has to expand each `(element, count)` entry into copies, lazily and from both ends.
  `iter::repeat_n(x, n)` yields `n` copies but clones only `n - 1` times, because it hands out the
  original last. `iter::repeat(x).take(n)` clones `n` times and breaks the clone budget, and
  collecting the copies into a `Vec` inside `into_iter` clones everything up front.

`flat_map` already does this expansion at both ends, so the only obstacle is the closure type `F` in
`FlatMap<I, U, F>`. A closure that captures nothing coerces to a function pointer, so the field can
be declared with `F = fn((T, usize)) -> RepeatN<T>`. The struct itself then needs `T: Clone`:
`FlatMap` requires `U: IntoIterator`, and `RepeatN<T>` is an iterator only when `T: Clone`.

The coercion happens only because the closure is written where the field's type is expected. If you
first store the iterator in a `let`, its type is inferred from the closure and no longer matches the
field (E0308). Cast the closure explicitly in that case:
`.flat_map((|(x, n)| iter::repeat_n(x, n)) as fn(_) -> _)`.

### Alternatives

These replace the `IntoIter` part of the solution (adjust the `use` lines to match).

Flatten by hand, with one `RepeatN` for each end (`RepeatN::default()` is an empty one). When the
entries in the middle run out, each end must finish the copies the other end has started; otherwise
mixing `next` and `next_back` loses items.

```rust
/// Iterator that moves the elements out of a bag.
pub struct IntoIter<T> {
    groups: btree_map::IntoIter<T, usize>,
    // The copies of the entry taken from each end (an empty `RepeatN` when there is none).
    front: RepeatN<T>,
    back: RepeatN<T>,
}

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            if let Some(x) = self.front.next() {
                return Some(x);
            }
            match self.groups.next() {
                Some((x, n)) => self.front = iter::repeat_n(x, n),
                // Nothing left in the middle: finish what the back end started.
                None => return self.back.next(),
            }
        }
    }
}

impl<T: Clone> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        loop {
            if let Some(x) = self.back.next_back() {
                return Some(x);
            }
            match self.groups.next_back() {
                Some((x, n)) => self.back = iter::repeat_n(x, n),
                None => return self.front.next_back(),
            }
        }
    }
}

impl<T: Clone> IntoIterator for Bag<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            groups: self.counts.into_iter(),
            front: RepeatN::default(),
            back: RepeatN::default(),
        }
    }
}
```

Without `repeat_n`, keep at each end an element and the number of copies still to hand out. Clone
while more than one copy is left, and move the element out last.

```rust
/// Iterator that moves the elements out of a bag.
pub struct IntoIter<T> {
    groups: btree_map::IntoIter<T, usize>,
    // An element and how many of its copies are still to be handed out, at each end.
    front: Option<(T, usize)>,
    back: Option<(T, usize)>,
}

/// Hands out one copy: a clone while more than one is left, and the original last.
fn take_one<T: Clone>(slot: &mut Option<(T, usize)>) -> Option<T> {
    let (x, n) = slot.take()?;
    if n > 1 {
        let copy = x.clone();
        *slot = Some((x, n - 1));
        Some(copy)
    } else {
        Some(x)
    }
}

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.front.is_none() {
            self.front = self.groups.next().or_else(|| self.back.take());
        }
        take_one(&mut self.front)
    }
}

impl<T: Clone> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.back.is_none() {
            self.back = self.groups.next_back().or_else(|| self.front.take());
        }
        take_one(&mut self.back)
    }
}

impl<T: Clone> IntoIterator for Bag<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter { groups: self.counts.into_iter(), front: None, back: None }
    }
}
```
