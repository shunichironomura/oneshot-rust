A small network protocol sends packets as raw bytes. Implement the conversions between bytes and
the types below (they are already in the editor), and a function that splits a stream into
packets.

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

impl TryFrom<u8> for Kind { /* type Error = PacketError; */ }
impl<'a> TryFrom<&'a [u8]> for Packet<'a> { /* type Error = PacketError; */ }

impl Packet<'_> {
    pub fn encode(&self) -> Vec<u8>;
}

pub fn decode_all(stream: &[u8]) -> Result<Vec<Packet<'_>>, PacketError>
```

A packet is a 9-byte header followed by the payload:

| Bytes | Field | Encoding |
| --- | --- | --- |
| `0..2` | magic | the two bytes `b"OS"` (`0x4F`, `0x53`) |
| `2` | kind | `1` is `Ping`, `2` is `Data`, `3` is `Ack` |
| `3..5` | `seq` | `u16`, little-endian |
| `5..9` | payload length | `u32`, **big-endian** |
| `9..` | `payload` | the payload bytes |

- `Kind::try_from(b)` converts `1`, `2` and `3` to `Ping`, `Data` and `Ack`. Any other `b` gives
  `Err(PacketError::UnknownKind(b))`.
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
- `decode_all(stream)` decodes packets that are stored back to back: the first packet starts at
  the beginning of `stream`, and each following packet starts right after the payload of the
  previous one. It returns all the packets in order. An empty `stream` gives `Ok(vec![])`.
  - Each packet is checked like in `Packet::try_from`, except for check 4. The payload is the
    next `declared` bytes after the header, and the bytes after it belong to the next packet. If
    fewer than `declared` bytes are left after the header, the error is
    `LengthMismatch { declared, actual }`, where `actual` is the number of bytes left after the
    header.
  - In particular, if fewer than 9 bytes are left where a packet should start, the error is
    `TooShort`.
  - If any packet is invalid, the result is the error of the first invalid packet.
- No function may panic, whatever the input.

### Examples

```rust
let bytes = [b'O', b'S', 2, 0x34, 0x12, 0, 0, 0, 2, 0xAA, 0xBB];
Packet::try_from(&bytes[..])
// Ok(Packet { kind: Kind::Data, seq: 0x1234, payload: &[0xAA, 0xBB] })
Packet::try_from(&bytes[..10])  // Err(LengthMismatch { declared: 2, actual: 1 })
Packet::try_from(&bytes[..5])   // Err(TooShort)
Kind::try_from(4)               // Err(UnknownKind(4))

Packet { kind: Kind::Ack, seq: 7, payload: b"hi" }.encode()
// vec![b'O', b'S', 3, 7, 0, 0, 0, 0, 2, b'h', b'i']

let stream = [
    b'O', b'S', 1, 1, 0, 0, 0, 0, 0,       // Ping, seq 1, empty payload
    b'O', b'S', 2, 2, 0, 0, 0, 0, 1, b'!', // Data, seq 2, payload b"!"
];
decode_all(&stream)
// Ok(vec![
//     Packet { kind: Kind::Ping, seq: 1, payload: &[] },
//     Packet { kind: Kind::Data, seq: 2, payload: b"!" },
// ])
```
