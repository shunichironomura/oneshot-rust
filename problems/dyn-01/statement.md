The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

`Unit` is a trait for the members of a game party, implemented by `Knight` and `Archer`.
`total_power` and `strongest` take a party with mixed kinds of units as `&[Box<dyn Unit>]`. They
return the sum of the units' power and the name of the strongest unit (the first one on a tie, and
`None` for an empty party).

```rust
pub fn total_power(party: &[Box<dyn Unit>]) -> u32
pub fn strongest(party: &[Box<dyn Unit>]) -> Option<String>
```

The harness also calls `promoted` and `beats` directly on `Knight` and `Archer` values, so keep
those methods usable there. Do not change the signatures of `total_power` and `strongest`.

### Example

For a party of `Knight { level: 2 }` (power 20) and `Archer { level: 3 }` (power 21),
`total_power` returns `41` and `strongest` returns `Some("archer(lv3)")`.
`Knight { level: 1 }.promoted()` is a level-2 knight, and it `beats` a level-2 archer.
