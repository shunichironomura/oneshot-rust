A traffic light is modeled by the `Light` enum below (it is already in the editor). Implement its
three methods.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
    Flashing(u32),
}

impl Light {
    pub fn next(self) -> Light;
    pub fn duration_secs(self) -> u32;
    pub fn time_until_green(self) -> u32;
}
```

- `next` returns the state that follows: `Green` → `Yellow` → `Red` → `Green`. `Flashing(n)` is a
  flashing yellow with `n` more flashes to go after the current one: `Flashing(n)` becomes
  `Flashing(n - 1)`, and `Flashing(0)` becomes `Red`.
- `duration_secs` returns how long the state lasts: `Red` 30, `Yellow` 5, `Green` 25, and 1 for every
  `Flashing` state.
- `time_until_green` returns how many seconds pass from the start of the current state until the
  light turns `Green`. It is `0` if the light is already `Green`.

### Examples

- `Light::Green.next()` is `Light::Yellow`.
- `Light::Flashing(2).next()` is `Light::Flashing(1)`, and `Light::Flashing(0).next()` is `Light::Red`.
- `Light::Yellow.time_until_green()` is `35` (5 seconds of yellow, then 30 of red).
