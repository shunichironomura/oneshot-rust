`next` is the only method you have to write. `take`, `sum`, `collect`, `count`, `max` and the other
adapters are provided methods of `Iterator` built on top of it.

The tricky part is the end of the sequence. Storing the value to return next as an `Option<u64>`
covers all three cases at once: `None` for an empty sequence, `None` again after `1`, and `self.next?`
returns early when there is nothing left. A plain `u64` with `0` as "finished" also works, but the
`Option` says what it means.

### Alternatives

`std::iter::successors` builds the same sequence from a first value and a step function, without a
struct of your own. It does not fit the given `Collatz` type, though, so you would wrap it.

```rust
pub struct Collatz {
    inner: Box<dyn Iterator<Item = u64>>,
}

impl Collatz {
    pub fn new(start: u64) -> Self {
        let first = (start != 0).then_some(start);
        let inner = std::iter::successors(first, |&n| match n {
            1 => None,
            n if n % 2 == 0 => Some(n / 2),
            n => Some(3 * n + 1),
        });
        Collatz { inner: Box::new(inner) }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
```
