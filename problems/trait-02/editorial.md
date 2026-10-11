`impl<I> Pairs for I where I: Iterator, I::Item: Clone` is a **blanket impl**: one impl gives
`pairs` to `vec::IntoIter`, `Chars`, ranges and every other iterator, as long as `Pairs` is in scope.
`pairs` only wraps `self` in a `PairsIter`; all the work happens lazily in `next`.

`PairsIter` has to remember the second item of the last pair, because that item is also the first
item of the next pair. Store it as `Option<I::Item>`; the item type is an associated type of `I`, so
the struct needs no extra type parameter. One copy goes into the returned pair and one stays behind,
which is why `I::Item: Clone` is required. `?` ends the iteration as soon as the inner iterator runs
out.

### Alternatives

`Peekable` already knows how to look one item ahead. Store it instead of the raw iterator and clone
the peeked item.

```rust
use std::iter::Peekable;

pub trait Pairs: Iterator + Sized {
    fn pairs(self) -> PairsIter<Self>;
}

pub struct PairsIter<I: Iterator> {
    iter: Peekable<I>,
}

impl<I> Pairs for I
where
    I: Iterator,
    I::Item: Clone,
{
    fn pairs(self) -> PairsIter<I> {
        PairsIter {
            iter: self.peekable(),
        }
    }
}

impl<I> Iterator for PairsIter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.iter.next()?;
        let second = self.iter.peek()?.clone();
        Some((first, second))
    }
}
```
