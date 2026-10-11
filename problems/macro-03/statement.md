Write a declarative macro `named_enum!` that defines an enum whose variants know their names. The
macro is in the editor with `#[macro_export]` and a rule that only defines the enum so far; make it
generate the rest. The error type `UnknownName` is given (it is already in the editor).

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownName(pub String);

#[macro_export]
macro_rules! named_enum { /* ... */ }
```

`named_enum! { Name { A, B = "b", C } }` takes the name of the enum and, in braces, its variants
separated by commas. A variant is an identifier, optionally followed by `= "label"` with a string
literal. There may be any number of variants, including none (`named_enum! { Never {} }`), and a
trailing comma is allowed. The call expands to these items:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Name { A, B, C }

impl Name {
    pub const ALL: &'static [Name];
    pub fn name(self) -> &'static str;
    pub fn from_name(name: &str) -> Option<Name>;
}

impl std::fmt::Display for Name { /* ... */ }

impl std::str::FromStr for Name {
    type Err = UnknownName;
    /* ... */
}
```

- The name of a variant is its label if it has one, and its identifier otherwise. Above, the names
  are `"A"`, `"b"` and `"C"`. No two variants of one enum have the same name.
- `ALL` contains every variant once, in the order they are written.
- `name()` returns the name of the variant.
- `from_name(s)` returns the variant whose name is exactly `s` (case-sensitive, nothing trimmed), or
  `None` if there is none. The identifier of a variant with a label is not a name.
- `Display` writes the name, so `to_string()` returns the same string as `name()`.
- `s.parse::<Name>()` returns `Ok` with the variant whose name is exactly `s`, or
  `Err(UnknownName(s.to_string()))`, which holds `s` unchanged.

Your code is the module `crate::user`. The tests are other modules of the same crate, outside
`crate::user`. They call the macro by its path, `crate::named_enum! { ... }`, at the item level of
their own modules (`#[macro_export]` puts the macro at the root of the crate). Such a module calls
the macro several times, has no `use` declarations for `named_enum`, `UnknownName` or anything from
the standard library, and may define its own `type Result<T> = std::result::Result<T, String>;`. The
generated items are also used from other modules.

### Examples

```rust
crate::named_enum! { Color { Red, Green, Blue } }
crate::named_enum! { Dir { North = "N", East = "E", South = "S", West = "W", } }

Color::ALL                // &[Color::Red, Color::Green, Color::Blue]
Color::Green.name()       // "Green"
Color::from_name("Blue")  // Some(Color::Blue)
Color::from_name("blue")  // None
Color::Red.to_string()    // "Red"
"Green".parse::<Color>()  // Ok(Color::Green)
"Purple".parse::<Color>() // Err(UnknownName("Purple".to_string()))

Dir::ALL                  // &[Dir::North, Dir::East, Dir::South, Dir::West]
Dir::West.name()          // "W"
Dir::from_name("W")       // Some(Dir::West)
Dir::from_name("West")    // None
Dir::East.to_string()     // "E"
"S".parse::<Dir>()        // Ok(Dir::South)
"South".parse::<Dir>()    // Err(UnknownName("South".to_string()))
```
