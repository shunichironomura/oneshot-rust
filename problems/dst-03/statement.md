A `Packet` is an id followed by a payload. In the code below, a packet made by `label` can only hold
a `String`. The test harness (the `tests.rs` tab) expects a new design: `label` accepts any value
that implements `Display`, and the packets it makes all have the same type, so that they fit in one
`Vec`.

The bodies are already written for the new design, but the signatures are left over from the old
one, so the harness does not compile. Change **only the signatures** (the `struct` line and the `fn`
lines) so that the harness compiles and passes. Do not change the bodies or the fields.

```rust
pub struct Packet<T> {
    pub id: u32,
    pub payload: T,
}

pub fn label(id: u32, payload: String) -> Box<Packet<String>>
pub fn describe(ps: &[Box<Packet<String>>]) -> Vec<String>
```

- `label(id, payload)` moves a value into a new boxed packet. The harness passes integers,
  floating-point numbers, characters, `bool`s, string literals and `String`s. Packets made by
  `label` all have the same type, whatever the type of their payload.
- `describe(ps)` formats each packet as `#<id>: <payload>`, with the payload formatted by `{}`
  (`Display`). The harness passes a `Vec` of packets made by `label`, or a part of one, such as
  `&ls[1..]`. The `Vec` may also contain a packet that the harness boxes itself, such as
  `Box::new(Packet { id: 3, payload: 'q' })`.

The harness also reads the fields `id` and `payload` of the packets.

### Examples

```rust
let ls = vec![label(1, 5), label(2, "x")];
describe(&ls)  // ["#1: 5", "#2: x"]
```
