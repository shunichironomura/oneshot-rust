`Packet<[u8]>` is a **custom dynamically sized type** (DST): a struct whose last field is unsized.
Like `[u8]` itself, it can only be used behind a pointer, and a pointer to it, such as
`Box<Packet<[u8]>>` or `&Packet<[u8]>`, is a fat pointer that stores the length of the payload next
to the address. With `Box<Packet<[u8]>>`, the id and the bytes live in a single allocation. Every
type parameter is `Sized` by default, so the struct needs `T: ?Sized` before `Packet<[u8]>` can even
be written. Only the last field may be unsized, and `payload` is the last one.

A `Packet<[u8]>` cannot be built directly: a struct expression needs a value for every field, and a
value of type `[u8]` cannot be passed around on its own. Instead, it is made from a sized packet by
an **unsizing coercion**. Behind a pointer, `Packet<[u8; N]>` coerces to `Packet<[u8]>`, because
`[u8; N]` coerces to `[u8]` and the type parameter appears only in the last field. That is why the
bodies work unchanged:

- `make` takes `[u8; N]` with a const generic `N`, so it accepts arrays of any length. The body
  builds a `Box<Packet<[u8; N]>>`, and the return type `Box<Packet<[u8]>>` coerces it, which turns
  the length `N` into run-time data. With `-> Box<Packet<[u8; N]>>`, `make` itself compiles, but
  `make(1, [1, 2, 3])` and `make(2, [9; 5])` then have different types, so
  `vec![make(1, [1, 2, 3]), make(2, [9; 5])]` does not compile.
- `checksum` takes `&Packet<[u8]>`. A `&Box<Packet<[u8]>>` reaches it by deref coercion, and
  `&Packet { id: 7, payload: [5u8, 6] }` by an unsizing coercion at the call. A const generic
  `checksum` taking `&Packet<[u8; N]>` would accept only the second kind.
