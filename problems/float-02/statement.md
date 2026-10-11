This code compiles, and it may even seem to work when you run it natively. But it contains
**undefined behavior (UB)**, and Miri detects it.

`to_levels` converts brightness samples into 8-bit color levels from 0 to 255. It returns one level
per sample, in the same order.

```rust
pub fn to_levels(samples: &[f32]) -> Vec<u8>
```

- A sample `s` becomes the level `(s * 255.0).round()`. `round` is `f32::round`, which rounds
  halves away from zero: `0.5` gives `127.5`, which becomes `128`.
- A level below 0 becomes 0, and a level above 255 becomes 255. A NaN sample becomes 0.
- Samples are **not** guaranteed to be in `0.0..=1.0`: they may be negative, larger than 1,
  infinite or NaN.

Remove the UB so that it works correctly for every input. (You may keep using `unsafe` if you
like.)

### Example

`to_levels(&[0.0, 0.5, 1.0, 1.5, -0.2, f32::NAN, f32::INFINITY])` returns
`[0, 128, 255, 255, 0, 0, 255]`.
