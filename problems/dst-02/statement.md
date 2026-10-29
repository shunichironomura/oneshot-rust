A `Packet` is an id followed by a payload. In the code below, a packet made by `make` keeps its
bytes in a separate `Vec<u8>`. The test harness (the `tests.rs` tab) expects a new design: `make`
moves a byte array into the packet itself, so that the id and the bytes share one allocation.

The bodies are already written for the new design, but the signatures are left over from the old
one, so the harness does not compile. Change **only the signatures** (the `struct` line and the `fn`
lines) so that the harness compiles and passes. Do not change the bodies or the fields.

```rust
pub struct Packet<T> {
    pub id: u32,
    pub payload: T,
}

pub fn make(id: u32, bytes: Vec<u8>) -> Box<Packet<Vec<u8>>>
pub fn checksum(p: &Packet<Vec<u8>>) -> u32
```

- `make(id, bytes)` moves a byte array of any length, including `[]`, into a new boxed packet.
  Packets made by `make` all have the same type, whatever the length of their payload: the harness
  keeps them together in one `Vec`, and changes their bytes through the box.
- `checksum(p)` returns the id plus the sum of the payload bytes. The harness passes packets made by
  `make`, and also packets with a byte array as payload that it creates itself, such as
  `&Packet { id: 7, payload: [5u8, 6] }`.

The harness also reads the fields `id` and `payload` of the packets.

### Examples

```rust
let a = make(1, [1, 2, 3]);
checksum(&a)                                    // 7 (1 + 1 + 2 + 3)
checksum(&Packet { id: 7, payload: [5u8, 6] })  // 18

let ps = vec![make(1, [1, 2, 3]), make(2, [9; 5]), make(3, [])];
ps.iter().map(|p| checksum(p)).collect::<Vec<_>>()  // [7, 47, 3]
```
