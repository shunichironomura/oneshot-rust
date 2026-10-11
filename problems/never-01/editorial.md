`field` is generic over the type `T` it parses, and the caller decides `T`. In `sum_ids`,
`let id: u32 = field(&cols, 0)?;` does that through the annotation. In `validate_row`, nothing does:
the value of `field(&cols, 1)?` is thrown away, and the return type `Result<(), BadField>` only
fixes the error type.

The message is about `!` rather than "type annotations needed" because of how `?` works. It expands
to a `match` whose error arm `return`s, and `return` has the never type `!`. When the type of a
`match` with such a diverging arm is not constrained by anything else, the compiler falls back to a
default type instead of giving up. The fallback used to be `()`; since Rust 1.100 it is `!` in
every edition (in Rust 2024 it already was). `!` does not implement `FromStr`, hence
``the trait bound `!: FromStr` is not satisfied``. Without the `?`, `field(&cols, 1);` would report
the usual "type annotations needed".

The fix is to name the type with a turbofish: `field::<u8>(&cols, 1)?`. Which type you name *is*
the validation, so it has to match the statement. `u32` for the age accepts `300`, `f64` for the id
accepts `1.5`, `i32` accepts `-1`, and `String` accepts anything. `field::<()>` does not compile
either, because `()` does not implement `FromStr` (so this code was rejected even when the fallback
was `()`).

`let _ = field(&cols, 1)?;` does not help: `_` is a pattern, not a type, so `T` is still unknown and
falls back to `!` again.

### Alternatives

A wildcard pattern with a type annotation does constrain `T`.

```rust
pub fn validate_row(line: &str) -> Result<(), BadField> {
    let cols: Vec<&str> = line.split(',').collect();
    let _: u32 = field(&cols, 0)?;
    let _: u8 = field(&cols, 1)?;
    let _: f64 = field(&cols, 2)?;
    Ok(())
}
```

You can also annotate the whole row once with a tuple type. The elements of a tuple expression are
evaluated from left to right, so the columns are still checked in order.

```rust
pub fn validate_row(line: &str) -> Result<(), BadField> {
    let cols: Vec<&str> = line.split(',').collect();
    let _row: (u32, u8, f64) = (field(&cols, 0)?, field(&cols, 1)?, field(&cols, 2)?);
    Ok(())
}
```
