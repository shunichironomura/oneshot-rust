A service writes its state to a log with `{:?}`. Passwords must never end up in the log, and some
fields are too big to be worth printing. Implement `Debug` by hand for the three types below (they
are already in the editor).

```rust
pub struct Credentials {
    pub user: String,
    pub password: String,
    pub scopes: Vec<String>,
}

pub struct Inventory {
    pub items: BTreeMap<String, u32>,
    pub cache: Vec<u8>,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl fmt::Debug for Credentials { /* to implement */ }
impl fmt::Debug for Inventory { /* to implement */ }
impl fmt::Debug for Color { /* to implement */ }
```

- Each type prints exactly what `#[derive(Debug)]` would print for the shape described below.
  Strings, lists and maps inside are shown the way `{:?}` shows a `String`, a `Vec` and a
  `BTreeMap`: strings in double quotes with special characters escaped, lists as `[...]`, and maps
  as `{key: value, ...}` in key order.
- `Credentials` looks like a struct named `Credentials` with the fields `user`, `password` and
  `scopes`, in this order. The value of `password` is always the bare text `<redacted>`, without
  quotes, whatever the password is (even an empty one).
- `Inventory` looks like a struct named `Inventory` with only the field `items`, followed by `..`
  to show that something was left out. `cache` is never printed, and the `..` is always there, even
  when `cache` is empty.
- `Color` looks like the tuple struct `Rgb(r, g, b)` when `a` is 255 (fully opaque), and like the
  tuple struct `Rgba(r, g, b, a)` otherwise.
- The output follows the format spec the same way a derived `Debug` does:
  - `{:?}` prints everything on one line, as in the examples.
  - `{:#?}` prints the pretty form. The name and the opening `{` or `(` are on the first line. Then
    each field is on its own line, indented by four spaces and followed by `,`. The `..` of
    `Inventory` gets its own line after the fields, without a `,`. The closing `}` or `)` is on the
    last line. Non-empty lists and maps inside are expanded in the same way, four spaces further
    in; empty ones stay `[]` and `{}`.
  - `{:x?}` and `{:X?}` print every integer inside in lowercase or uppercase hexadecimal, without
    a prefix (`ff`, `FF`). With `#` as well (`{:#x?}`, `{:#X?}`), the output is in the pretty form
    and every integer gets a `0x` prefix (`0xff`, `0xFF`, `0x0`).
  - All of this also holds when the value is printed as part of another value, for example as an
    element of a `Vec` printed with `{:#?}`. Its lines are then indented one more level, like any
    nested value.
- Width, fill, alignment, precision and the `+` and `0` flags are not used.

### Examples

```rust
let c = Credentials {
    user: "ann".to_string(),
    password: "hunter2".to_string(),
    scopes: vec!["read".to_string(), "write".to_string()],
};
```

`format!("{c:?}")` is

```text
Credentials { user: "ann", password: <redacted>, scopes: ["read", "write"] }
```

and `format!("{c:#?}")` is

```text
Credentials {
    user: "ann",
    password: <redacted>,
    scopes: [
        "read",
        "write",
    ],
}
```

For an `Inventory` whose `items` are `"pear"` → `1` and `"apple"` → `3`, and whose `cache` holds
64 bytes, `{:?}` and `{:#?}` print

```text
Inventory { items: {"apple": 3, "pear": 1}, .. }
```

```text
Inventory {
    items: {
        "apple": 3,
        "pear": 1,
    },
    ..
}
```

With `red = Color { r: 255, g: 0, b: 16, a: 255 }` and `glass = Color { r: 255, g: 0, b: 16, a: 128 }`:

| Code | Result |
| --- | --- |
| `format!("{red:?}")` | `Rgb(255, 0, 16)` |
| `format!("{red:x?}")` | `Rgb(ff, 0, 10)` |
| `format!("{glass:?}")` | `Rgba(255, 0, 16, 128)` |

and `format!("{red:#X?}")` is

```text
Rgb(
    0xFF,
    0x0,
    0x10,
)
```
