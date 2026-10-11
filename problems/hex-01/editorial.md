Look at characters, not bytes, and check every one of them before computing anything.

`strip_prefix('#')` returns the rest of `s` after a leading `#`, or `None` if there is none, and `?`
passes that `None` on. `char::to_digit(16)` gives the value of a hex digit and `None` for every
other `char`, non-ASCII ones included. Collecting an iterator of `Option`s into an `Option<Vec<_>>`
gives `None` as soon as one item is `None`. What is left is a list of digit values, and a slice
pattern accepts the two valid lengths. In the 3-digit form, `dd` is `16 * d + d`, that is `17 * d`.

The traps on the way:

- `u8::from_str_radix` (like `str::parse`) accepts an optional leading `+`, so
  `u8::from_str_radix("+f", 16)` is `Ok(15)`. Parsing `#+f+f+f` two characters at a time gives
  `[15, 15, 15]` instead of `None`, and parsing all six digits at once as a `u32` accepts
  `#+fff00`. Unsigned types reject `-`, but a signed type would accept it. Checking every character
  with `is_ascii_hexdigit` (or `to_digit`) first closes the hole.
- `s.len()` is the length in **bytes**. `#1é234` is 7 bytes long, like a 6-digit code, and
  `&s[1..3]` panics because byte 3 is in the middle of `é`. `#€` is 4 bytes long, like a 3-digit
  code, and `&s[1..2]` panics too. Slicing at byte offsets is only safe once you know that the
  string is ASCII, and `s.get(1..3)` returns `None` instead of panicking.
- `{:x}` writes no leading zero, so `[255, 8, 0]` would become `#ff80`. `{:02x}` pads every channel
  to two digits with zeros. For the short form, `c % 17 == 0` tells whether both digits of `c` are
  equal, and `c / 17` is that digit.

### Alternatives

Validate first, then parse with `u8::from_str_radix`. Once every byte is an ASCII hex digit, every
byte offset is a character boundary, and no sign can sneak in.

```rust
pub fn parse_hex_color(s: &str) -> Option<[u8; 3]> {
    let hex = s.strip_prefix('#')?;
    if !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let width = match hex.len() {
        6 => 2,
        3 => 1,
        _ => return None,
    };
    let mut rgb = [0; 3];
    for (i, channel) in rgb.iter_mut().enumerate() {
        let value = u8::from_str_radix(&hex[i * width..(i + 1) * width], 16).ok()?;
        *channel = if width == 1 { value * 17 } else { value };
    }
    Some(rgb)
}
```

`to_hex` can also be built digit by digit. `char::from_digit` writes lowercase letters, and in the
short form only the low digit of each channel is written.

```rust
pub fn to_hex(rgb: [u8; 3]) -> String {
    let short = rgb.iter().all(|&c| c >> 4 == c & 0xf);
    let digit = |n: u8| char::from_digit(u32::from(n), 16).unwrap();
    let mut out = String::from("#");
    for c in rgb {
        if !short {
            out.push(digit(c >> 4));
        }
        out.push(digit(c & 0xf));
    }
    out
}
```
