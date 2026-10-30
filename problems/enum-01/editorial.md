`match` must cover every variant, so forgetting one is a compile error rather than a silent bug.
Arms are tried from top to bottom: `Light::Flashing(0)` has to come before `Light::Flashing(n)`,
otherwise `n - 1` would underflow for `0` (a panic in debug builds). `Light::Flashing(_)` matches any
count when the number does not matter.

`Light` derives `Copy`, so the methods can take `self` by value and `light = light.next()` simply
replaces the old value. `time_until_green` adds up durations until it reaches `Green`. Every state
eventually leads to `Green`, so the loop ends.

### Alternatives

A match guard expresses the same split between zero and non-zero counts.

```rust
pub fn next(self) -> Light {
    match self {
        Light::Red => Light::Green,
        Light::Green => Light::Yellow,
        Light::Yellow => Light::Red,
        Light::Flashing(n) if n > 0 => Light::Flashing(n - 1),
        Light::Flashing(_) => Light::Yellow,
    }
}
```

`time_until_green` can also be written with recursion.

```rust
pub fn time_until_green(self) -> u32 {
    match self {
        Light::Green => 0,
        other => other.duration_secs() + other.next().time_until_green(),
    }
}
```
