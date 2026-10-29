The code below does not compile. Fix it.

The function returns a regular expression for decimal numbers such as `3.14`, written as a literal
in its body.

```rust
pub fn number_pattern() -> &'static str
```

The result must be exactly the 8 characters `\d+\.\d+`. This is the text itself, not a Rust
literal: every `\` is one backslash character.

Do not change the signature.

### Examples

- `number_pattern()` returns the 8 characters `\`, `d`, `+`, `\`, `.`, `\`, `d`, `+`.
