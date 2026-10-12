A small network protocol sends packets as raw bytes. Implement the conversions between bytes and
the types below (they are already in the editor).

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Ping = 1,
    Data = 2,
    Ack = 3,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PacketError {
    TooShort,
    BadMagic,
    UnknownKind(u8),
    LengthMismatch { declared: u32, actual: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Packet<'a> {
    pub kind: Kind,
    pub seq: u16,
    pub payload: &'a [u8],
}

impl TryFrom<u8> for Kind { /* already implemented */ }
impl<'a> TryFrom<&'a [u8]> for Packet<'a> { /* type Error = PacketError; */ }

impl Packet<'_> {
    pub fn encode(&self) -> Vec<u8>;
}
```

A packet is a 9-byte header followed by the payload:

| Bytes | Field | Encoding |
| --- | --- | --- |
| `0..2` | magic | the two bytes `b"OS"` (`0x4F`, `0x53`) |
| `2` | kind | `1` is `Ping`, `2` is `Data`, `3` is `Ack` |
| `3..5` | `seq` | `u16`, little-endian |
| `5..9` | payload length | `u32`, **big-endian** |
| `9..` | `payload` | the payload bytes |

- `Kind::try_from(b)` is already implemented: it converts `1`, `2` and `3` to `Ping`, `Data` and
  `Ack`, and any other `b` to `Err(PacketError::UnknownKind(b))`.
- `Packet::try_from(bytes)` decodes a slice that holds exactly one packet. It runs the following
  checks in this order and returns the error of the first check that fails:
  1. `bytes` is shorter than 9 bytes: `TooShort`.
  2. The first two bytes are not `b"OS"` (case matters): `BadMagic`.
  3. The kind byte `k` is not `1`, `2` or `3`: `UnknownKind(k)`.
  4. The number of bytes after the header is not equal to the payload length in the header:
     `LengthMismatch { declared, actual }`, where `declared` is the payload length in the header
     and `actual` is the number of bytes after the header.

  If every check passes, it returns the packet. Its `payload` is the part of `bytes` after the
  header.
- `encode` does the reverse: it returns the header followed by the payload, so that
  `Packet::try_from(&p.encode()[..])` gives back `Ok(p)`. You may assume that the payload is at
  most `u32::MAX` bytes long.
- No function may panic, whatever the input.

### Examples

```rust
let bytes = [b'O', b'S', 2, 0x34, 0x12, 0, 0, 0, 2, 0xAA, 0xBB];
Packet::try_from(&bytes[..])
// Ok(Packet { kind: Kind::Data, seq: 0x1234, payload: &[0xAA, 0xBB] })
Packet::try_from(&bytes[..10])  // Err(LengthMismatch { declared: 2, actual: 1 })
Packet::try_from(&bytes[..5])   // Err(TooShort)

Packet { kind: Kind::Ack, seq: 7, payload: b"hi" }.encode()
// vec![b'O', b'S', 3, 7, 0, 0, 0, 0, 2, b'h', b'i']
```
