Several threads produce numbers, and one channel of `std::sync::mpsc` gathers them all. Implement
the function `fan_in`.

```rust
pub fn fan_in(groups: Vec<Vec<u32>>) -> Vec<u32>
```

- `fan_in` creates one channel and spawns one thread per group. Each thread sends the numbers of its
  group through its own clone of the channel's `Sender`.
- `fan_in` receives every number, joins all the threads, and returns the numbers sorted in ascending
  order, keeping duplicates.
- `groups` may be empty, and so may any group.

Do not change the signature.

### Examples

`fan_in(vec![vec![3, 1], vec![2]])` returns `vec![1, 2, 3]`.
