Add a `pairs` method to **every iterator** whose items can be cloned. It returns a new iterator that
yields each item together with the item right after it.

```rust
pub trait Pairs: Iterator + Sized {
    fn pairs(self) -> PairsIter<Self>;
}
```

The skeleton in `template.rs` already has the trait, the `PairsIter` struct and the impls. Fill in the
bodies, and add fields to `PairsIter` as you need. Do not change the trait.

- An iterator with fewer than two items yields no pairs.
- `pairs` must be lazy: it must also work on an infinite iterator such as `(0..)`.

### Example

`vec![1, 2, 3, 4].into_iter().pairs()` yields `(1, 2)`, `(2, 3)`, `(3, 4)`.
`"abc".chars().pairs()` yields `('a', 'b')`, `('b', 'c')`.
