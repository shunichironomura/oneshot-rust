The obvious body does not compile:

```rust
let (run, tail) = self.rest.split_at_mut(len);
self.rest = tail;
Some(run)
```

`next` only gets `&mut self`, a borrow that lasts for some short lifetime `'1` of this call.
Through it, `self.rest` can only be **reborrowed**, and a reborrow cannot outlive the borrow it goes
through: `split_at_mut` hands out two `&'1 mut [T]`, not `&'a mut [T]`, and rustc reports "method
was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`". This
rule is what keeps `&mut` unique: otherwise the caller would get a `&'a mut` to elements that
`self.rest` still points to, and the compiler does not reason that you overwrite `self.rest` right
after. Moving the slice out with `let rest = self.rest;` fails too, because `&mut [T]` is not
`Copy` and `self` is only borrowed. (Over `&'a [T]`, which is `Copy`, the obvious body would work.)

The way out is to take the `&'a mut [T]` out of `self` **by value** and leave something valid in
its place. `std::mem::take(&mut self.rest)` does exactly that: `&mut [T]` implements `Default` as an
empty slice, which borrows nothing and so fits any lifetime. The function now owns `rest`, a
`&'a mut [T]`; `split_at_mut` consumes it and returns two non-overlapping `&'a mut [T]`, and the
part that is not yielded goes back into `self.rest`. `next_back` does the same from the other end.
This is how the Rustonomicon writes `IterMut` for slices ("Splitting Borrows"), without `unsafe`.

The length of the run is computed before the take, through a short shared borrow. Start at 1 and
compare neighbors (`windows(2)`), so a run is never empty. Counting with
`iter().take_while(|x| *x == &rest[0])` looks equivalent, but it also compares the first element
with itself: for a NaN it counts 0, and the iterator yields empty runs forever. `filter` instead of
`take_while` would also count equal values further away.

### Alternatives

`std::mem::replace(&mut self.rest, &mut [])` is the same trick with the empty slice written out.
`position` and `rposition` find the first and the last pair of neighbors that differ.

```rust
fn next(&mut self) -> Option<&'a mut [T]> {
    let rest = mem::replace(&mut self.rest, &mut []);
    if rest.is_empty() {
        return None;
    }
    // The run ends at the first pair of neighbors that are not equal.
    let len = rest.windows(2).position(|w| w[0] != w[1]).map_or(rest.len(), |i| i + 1);
    let (run, tail) = rest.split_at_mut(len);
    self.rest = tail;
    Some(run)
}

fn next_back(&mut self) -> Option<&'a mut [T]> {
    let rest = mem::replace(&mut self.rest, &mut []);
    if rest.is_empty() {
        return None;
    }
    // The last run starts right after the last pair of neighbors that are not equal.
    let start = rest.windows(2).rposition(|w| w[0] != w[1]).map_or(0, |i| i + 1);
    let (head, run) = rest.split_at_mut(start);
    self.rest = head;
    Some(run)
}
```

The problem only exists behind `&mut self`. A **local** `&'a mut [T]` can be split and overwritten
directly: nothing else can reach it, and assigning `rest = tail` does not touch the elements. So
the constructor can split everything up front, and the iterator just pops from a `VecDeque`. The
price is an allocation and a pass over the whole slice before the first run.

```rust
use std::collections::VecDeque;

pub struct RunsMut<'a, T> {
    runs: VecDeque<&'a mut [T]>,
}

pub fn runs_mut<T: PartialEq>(v: &mut [T]) -> RunsMut<'_, T> {
    let mut runs = VecDeque::new();
    let mut rest = v;
    while !rest.is_empty() {
        let len = 1 + rest.windows(2).take_while(|w| w[0] == w[1]).count();
        let (run, tail) = rest.split_at_mut(len);
        runs.push_back(run);
        rest = tail;
    }
    RunsMut { runs }
}

impl<'a, T: PartialEq> Iterator for RunsMut<'a, T> {
    type Item = &'a mut [T];

    fn next(&mut self) -> Option<&'a mut [T]> {
        self.runs.pop_front()
    }
}

impl<'a, T: PartialEq> DoubleEndedIterator for RunsMut<'a, T> {
    fn next_back(&mut self) -> Option<&'a mut [T]> {
        self.runs.pop_back()
    }
}
```

Finally, the standard library has this iterator: `chunk_by_mut` (stable since Rust 1.77) returns a
double-ended `ChunkByMut`, whose implementation is essentially the model solution. To store it in
the struct, the predicate needs a type you can name, such as a function pointer.

```rust
use std::slice::ChunkByMut;

pub struct RunsMut<'a, T> {
    inner: ChunkByMut<'a, T, fn(&T, &T) -> bool>,
}

pub fn runs_mut<T: PartialEq>(v: &mut [T]) -> RunsMut<'_, T> {
    let same: fn(&T, &T) -> bool = |a, b| a == b;
    RunsMut { inner: v.chunk_by_mut(same) }
}

impl<'a, T: PartialEq> Iterator for RunsMut<'a, T> {
    type Item = &'a mut [T];

    fn next(&mut self) -> Option<&'a mut [T]> {
        self.inner.next()
    }
}

impl<'a, T: PartialEq> DoubleEndedIterator for RunsMut<'a, T> {
    fn next_back(&mut self) -> Option<&'a mut [T]> {
        self.inner.next_back()
    }
}
```
