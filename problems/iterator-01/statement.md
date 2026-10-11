Make `Collatz::new(start)` a sequence that can be used with `for` loops and methods such as `take`,
`sum` and `collect`.

```rust
pub struct Collatz { /* up to you */ }

impl Collatz {
    pub fn new(start: u64) -> Self;
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item>;
}
```

- The sequence starts with `start`. After a value `n`, the next value is `n / 2` if `n` is even and
  `3 * n + 1` if `n` is odd.
- The sequence ends right after `1` (`1` is included). After that, `next` keeps returning `None`.
- `Collatz::new(0)` is an empty sequence.
- Compute each value only when it is requested. Do not build the whole sequence up front.

### Examples

| Expression | Result |
| --- | --- |
| `Collatz::new(6).collect::<Vec<_>>()` | `[6, 3, 10, 5, 16, 8, 4, 2, 1]` |
| `Collatz::new(1).collect::<Vec<_>>()` | `[1]` |
