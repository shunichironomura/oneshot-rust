Each function here is short with the right adapter, and subtly wrong with an adapter that only
looks right.

`position` returns how many items came before the match in the iterator it is called on, so after
`rev()` it counts from the back. `words.iter().rev().position(|w| w.chars().count() >= 5)` returns
`Some(0)` for `["tea", "coffee", "milk", "juice"]`, because `"juice"` is the first item of the
reversed iterator. `rposition` searches from the back too, but returns the position from the front.
It needs an iterator that knows its length and can be walked from the back (`ExactSizeIterator` and
`DoubleEndedIterator`), as `slice::Iter` does. Count with `chars().count()`: `len()` is the length
in bytes, and `"日本"` is 6 bytes long but only 2 characters.

`take_while` has to look at the first line that fails the test to know where to stop, and that line
is then gone from the underlying iterator. This loses `"body"`:

```rust
let mut it = lines.iter().copied();
let header: Vec<_> = it.by_ref().take_while(|l| l.starts_with('#')).collect();
let body: Vec<_> = it.collect(); // ["# x"]
```

Find the boundary with `position` and cut the slice with `split_at` instead. (`filter` or
`partition` would move `"# x"` into the header.)

`scan` is a `map` that carries a mutable state. Its closure returns an `Option`, and `None` ends the
iteration, which is exactly "stop at the first rejected transaction". The closure of `filter_map`
returns an `Option` too, but there `None` only skips that transaction and the iteration goes on,
giving `[70, 120, 130]`.

`iter::successors` builds a sequence in which each item is computed from the previous one, and ends
when the closure returns `None`. `checked_mul` returns `None` exactly when the product would
overflow, so the sequence ends after the largest power that fits. With `p * base`, the overflow
panics ("attempt to multiply with overflow") in a debug build such as the judge's, and silently
wraps around in a release build. Write the start as `Some(1u64)`: with `Some(1)`, the type of `p` is
not known yet when the closure calls `checked_mul` on it, and that is a compile error. Floating-point
logarithms are no way out either: `u64::MAX as f64` rounds up to exactly 2⁶⁴, so for a base such as
`2` or `16` the logarithm comes out as exactly `64` or `16`, and 2⁶⁴, which does not fit, is
counted too.

### Alternatives

`enumerate` before `rev` attaches the positions from the front, and they stay attached when the
order is reversed. (`rev().enumerate()` would number the items from the back again.)

```rust
pub fn last_long(words: &[&str], min: usize) -> Option<usize> {
    words
        .iter()
        .enumerate()
        .rev()
        .find(|(_, w)| w.chars().count() >= min)
        .map(|(i, _)| i)
}
```

`Peekable::next_if` returns the next item only if it passes the test, and otherwise leaves it in
place, so nothing is lost.

```rust
pub fn split_header<'a>(lines: &[&'a str]) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut it = lines.iter().copied().peekable();
    let mut header = Vec::new();
    while let Some(line) = it.next_if(|line| line.starts_with('#')) {
        header.push(line);
    }
    (header, it.collect())
}
```

You can also compute the running sums and keep them up to the first negative one. The state does add
the rejected transaction, but `take_while` drops that sum and then stops asking `scan` for more
items, so no later transaction is added.

```rust
pub fn running_balance(start: i64, txns: &[i64]) -> Vec<i64> {
    txns.iter()
        .scan(start, |balance, &t| {
            *balance += t;
            Some(*balance)
        })
        .take_while(|&b| b >= 0)
        .collect()
}
```

`checked_pow` with `map_while` (a `map` that stops at the first `None`) works too. The exponent
`0..` is a `u32`, the type `checked_pow` takes.

```rust
pub fn powers_of(base: u64) -> Vec<u64> {
    (0..).map_while(|e| base.checked_pow(e)).collect()
}
```

Or compute the largest exponent first with the exact integer logarithm `ilog`.

```rust
pub fn powers_of(base: u64) -> Vec<u64> {
    (0..=u64::MAX.ilog(base)).map(|e| base.pow(e)).collect()
}
```
