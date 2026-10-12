`split_at(n)` would cut the payload off the rest of the stream, but it panics if `n` is larger
than the slice. `split_at_checked` returns `None` instead, which `ok_or` turns into the
`LengthMismatch`. Each step only moves the `rest` window forward; no bytes are copied.

The declared length is a `u32` and slice lengths are `usize`. `declared as usize` widens the length
(nothing is lost on 32- and 64-bit targets), whereas `after_header.len() as u32` would cut off
lengths of 4 GiB and more. Also avoid computing `9 + declared` as a `u32`: for a length near
`u32::MAX` it overflows, which panics in debug builds such as the judge's.

Every payload is a part of `stream`, so the packets borrow from it. In
`decode_all(stream: &[u8]) -> Result<Vec<Packet<'_>>, PacketError>`, the `'_` is the lifetime of
`stream`: with a single reference parameter, elision ties the output to it. The same rule ties the
slice returned by `split_header` to `bytes`, so `after_header` and `rest` are parts of `stream`
too.

### Alternatives

`get(..n)` also returns `None` for a range past the end. After it succeeds, `after_header[n..]`
cannot panic.

```rust
pub fn decode_all(stream: &[u8]) -> Result<Vec<Packet<'_>>, PacketError> {
    let mut packets = Vec::new();
    let mut rest = stream;
    while !rest.is_empty() {
        let (Header { kind, seq, declared }, after_header) = split_header(rest)?;
        let n = declared as usize;
        let Some(payload) = after_header.get(..n) else {
            return Err(PacketError::LengthMismatch { declared, actual: after_header.len() });
        };
        packets.push(Packet { kind, seq, payload });
        rest = &after_header[n..];
    }
    Ok(packets)
}
```
