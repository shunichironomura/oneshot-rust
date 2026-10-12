A small network protocol sends packets as raw bytes, back to back in one stream. Implement
`decode_all`, which splits a stream into packets. The types below and a header parser are already
in the editor.

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

struct Header {
    kind: Kind,
    seq: u16,
    declared: u32,
}

fn split_header(bytes: &[u8]) -> Result<(Header, &[u8]), PacketError> // already implemented

pub fn decode_all(stream: &[u8]) -> Result<Vec<Packet<'_>>, PacketError>
```

A packet is a 9-byte header followed by the payload:

| Bytes | Field | Encoding |
| --- | --- | --- |
| `0..2` | magic | the two bytes `b"OS"` |
| `2` | kind | `1` is `Ping`, `2` is `Data`, `3` is `Ack` |
| `3..5` | `seq` | `u16`, little-endian |
| `5..9` | payload length (`declared`) | `u32`, big-endian |
| `9..` | `payload` | the next `declared` bytes |

- `split_header(bytes)` reads the header at the start of `bytes`. It returns the error `TooShort`
  if `bytes` is shorter than 9 bytes, `BadMagic` if the first two bytes are not `b"OS"`, and
  `UnknownKind(k)` if the kind byte `k` is not `1`, `2` or `3`, checked in this order. Otherwise
  it returns the header and all the bytes after it.
- `decode_all(stream)` decodes the packets in `stream`: the first packet starts at the beginning
  of `stream`, and each following packet starts right after the payload of the previous one.
  When no bytes are left after a packet, decoding stops and the function returns all the packets
  in order. An empty `stream` gives `Ok(vec![])`.
- Each packet's header is checked as `split_header` does. If fewer than `declared` bytes are left
  after the header, the error is `LengthMismatch { declared, actual }`, where `actual` is the
  number of bytes left after the header.
- If any packet is invalid, the result is the error of the first invalid packet.
- `decode_all` must not panic, whatever the input.

### Examples

```rust
let stream = [
    b'O', b'S', 1, 1, 0, 0, 0, 0, 0,       // Ping, seq 1, empty payload
    b'O', b'S', 2, 2, 0, 0, 0, 0, 1, b'!', // Data, seq 2, payload b"!"
];
decode_all(&stream)
// Ok(vec![
//     Packet { kind: Kind::Ping, seq: 1, payload: &[] },
//     Packet { kind: Kind::Data, seq: 2, payload: b"!" },
// ])

let cut = [
    b'O', b'S', 1, 1, 0, 0, 0, 0, 0,       // Ping, seq 1, empty payload
    b'O', b'S', 2, 2, 0, 0, 0, 0, 3, b'!', // Data, seq 2, 3 bytes declared, 1 left
];
decode_all(&cut)        // Err(LengthMismatch { declared: 3, actual: 1 })
decode_all(&cut[..12])  // Err(TooShort)
```
