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

Normally the light cycles `Green` → `Yellow` → `Red` → `Green`. At times, such as late at night with
little traffic, it is switched to flashing yellow. That switch is made from outside, so none of `Red`,
`Yellow` and `Green` turns into `Flashing` through `next`. The flashing stops after a set number of
flashes and the light returns to its normal cycle. It does not jump from flashing straight to red: it
shows a steady `Yellow` first, then `Red`.

`Flashing(n)` is one flash of yellow, with `n` more flashes to go after the current one. `n` can be any
value. For example, `Flashing(2)` flashes three times, counting the current one.

- `next` returns the state that follows. `Flashing(n)` becomes `Flashing(n - 1)`, and `Flashing(0)`
  becomes `Yellow`.
- `duration_secs` returns how long the state lasts: `Red` 30, `Yellow` 5, `Green` 25, and 1 for every
  `Flashing` state.
- `time_until_green` returns how many seconds pass from the start of the current state until the
  light turns `Green`. It is `0` if the light is already `Green`.

### Examples

- `Light::Green.next()` is `Light::Yellow`.
- `Light::Flashing(2).next()` is `Light::Flashing(1)`, and `Light::Flashing(0).next()` is
  `Light::Yellow`.
- `Light::Yellow.time_until_green()` is `35` (5 seconds of yellow, then 30 of red).
- `Light::Flashing(0).time_until_green()` is `36` (1 second of flashing, 5 of yellow, then 30 of red).
