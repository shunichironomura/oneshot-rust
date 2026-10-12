`u16::from_le_bytes` and `u32::from_be_bytes` take an **array** (`[u8; 2]`, `[u8; 4]`) by value,
not a slice, so `u16::from_le_bytes(bytes[3..5])` does not compile. The solution gets arrays with
`split_first_chunk::<9>()`: it returns `None` if there are fewer than 9 bytes (check 1), and
otherwise `Some((&[u8; 9], &[u8]))`, the header as an array and the rest of the slice. An array
pattern on `*header` names all nine bytes, and `[s0, s1]` and `[l0, l1, l2, l3]` build the arrays
for the two numbers. Mind the byte order: `seq` is little-endian and the length big-endian.
`encode` writes them back with `to_le_bytes` and `to_be_bytes`.

If you index instead, check the length first: `bytes[2]` or `bytes[5..9]` panics on a short
slice, and checking the magic first (for example with `starts_with`) gives `BadMagic` instead of
`TooShort` for a short slice with the wrong bytes. `bytes[3..5].try_into()` does produce a
`[u8; 2]`, but its error type is `TryFromSliceError`, which `?` cannot turn into a `PacketError`.
After the length check the conversion cannot fail, so `unwrap()` is fine.

The declared length is a `u32` and slice lengths are `usize`, so they cannot be compared directly.
`declared as usize` widens the length (nothing is lost on 32- and 64-bit targets), whereas
`payload.len() as u32` would cut off lengths of 4 GiB and more. Also avoid computing
`9 + declared` as a `u32`: for a length near `u32::MAX` it overflows, which panics in debug builds
such as the judge's.

The payload is a part of the input, so a `Packet<'a>` borrows from it.
`impl<'a> TryFrom<&'a [u8]> for Packet<'a>` ties the two lifetimes together, which is why
`try_from` takes `bytes: &'a [u8]`. Writing `bytes: &[u8]` there gives "lifetime may not live long
enough". Finally, the discriminants of `Kind` are the wire values, so `self.kind as u8` gives
the kind byte.

### Alternatives

The snippets below use the constants `MAGIC` and `HEADER_LEN` of the solution.

Index into the slice once its length has been checked.

```rust
impl<'a> TryFrom<&'a [u8]> for Packet<'a> {
    type Error = PacketError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        if bytes.len() < HEADER_LEN {
            return Err(PacketError::TooShort);
        }
        if !bytes.starts_with(b"OS") {
            return Err(PacketError::BadMagic);
        }
        let kind = Kind::try_from(bytes[2])?;
        // The length was checked above, so these conversions cannot fail.
        let seq = u16::from_le_bytes(bytes[3..5].try_into().unwrap());
        let declared = u32::from_be_bytes(bytes[5..9].try_into().unwrap());
        let payload = &bytes[HEADER_LEN..];
        if payload.len() != declared as usize {
            return Err(PacketError::LengthMismatch { declared, actual: payload.len() });
        }
        Ok(Packet { kind, seq, payload })
    }
}
```

`encode` can join the parts with `concat`. The type annotation makes every element a `&[u8]`,
whatever the length of the array it points to.

```rust
pub fn encode(&self) -> Vec<u8> {
    let len = self.payload.len() as u32;
    let parts: [&[u8]; 5] = [
        &MAGIC,
        &[self.kind as u8],
        &self.seq.to_le_bytes(),
        &len.to_be_bytes(),
        self.payload,
    ];
    parts.concat()
}
```
