The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

Each of the three functions returns one fixed value, written as a literal in its body.

```rust
pub fn number_pattern() -> &'static str
pub fn hashtag_quote() -> &'static str
pub fn png_magic() -> &'static [u8]
```

The tests compare each result with the value below exactly, character by character (byte by byte
for `png_magic`). The values are shown as the characters themselves, not as Rust literals: every
`\` in the table is one backslash character.

| Function | Value | `len()` |
| --- | --- | --- |
| `number_pattern` | `\d+\.\d+`, a regular expression for numbers such as `3.14` | 8 |
| `hashtag_quote` | `He said "#rust" twice` | 21 |
| `png_magic` | the PNG file signature, the bytes `0x89`, `0x50`, `0x4E`, `0x47`, `0x0D`, `0x0A`, `0x1A`, `0x0A` | 8 |

Do not change the signatures.

### Examples

- `number_pattern()` returns the 8 characters `\`, `d`, `+`, `\`, `.`, `\`, `d`, `+`.
