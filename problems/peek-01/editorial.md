A tokenizer must look at the next character **without consuming it**: the run of digits in `42+y`
ends at `+`, and that `+` is the next token. `chars().peekable()` provides this. `peek` shows the
next item, and `next_if(pred)` takes it only when `pred` returns `true`, so
`while let Some((_, d)) = chars.next_if(...)` reads a run of digits and stops right in front of the
first non-digit. `->` works the same way: after a `-`, `next_if` takes a `>` if one follows.

`enumerate` before `peekable` attaches the position to every character, so `pos` is right without
manual counting. `char_indices` would give **byte** offsets instead, which differ from character
positions after a non-ASCII character such as U+3000.

The digits are collected into a `String` and parsed with `parse::<u64>()`. The string is never
empty and holds only ASCII digits, so the only possible error is a value that is too large.
If you compute the value yourself, `value * 10 + digit` panics on overflow in debug builds (and
under Miri); use `checked_mul` and `checked_add` instead, as the first alternative does. A wider
type such as `u128` does not fix it, because a number can have any number of digits.

Returning as soon as an error is found (`?` or `return Err(...)`) gives the first error for free.

Pitfalls:

- `chars.by_ref().take_while(...)` looks like the right tool, but it consumes the first character
  that fails the test and throws it away: `42+y` loses its `+`.
- `peek` takes `&mut self` and returns a reference into the iterator. In
  `while let Some(d) = chars.peek() { chars.next(); name.push(*d); }`, `d` is still in use when
  `next` is called, so the borrow checker rejects it (E0499). Copy the value out with the pattern
  `Some(&d)` (or use `next_if`, which needs no separate `next`).
- `next_if_eq` takes a reference: `next_if_eq(&'>')`, not `next_if_eq('>')`.
- `is_alphanumeric`, `is_alphabetic` and `is_numeric` accept non-ASCII letters and digits, and
  `is_ascii_whitespace` rejects U+3000. Use the `is_ascii_*` methods for tokens and
  `is_whitespace` for whitespace.

### Alternatives

`peek` with a manual position counter, and the value computed with `checked_mul` and
`checked_add`. Every `next` must be matched by `pos += 1`, which is easy to forget; this is what
`enumerate` saves.

```rust
pub fn tokenize(src: &str) -> Result<Vec<Token>, LexError> {
    let mut chars = src.chars().peekable();
    let mut pos = 0; // position of the next character `chars` yields
    let mut tokens = Vec::new();
    while let Some(c) = chars.next() {
        let start = pos;
        pos += 1;
        if c.is_whitespace() {
            continue;
        }
        let token = match c {
            '0'..='9' => {
                let mut value = u64::from(c) - u64::from('0');
                // `&d` copies the peeked char, so `chars` is free to move on.
                while let Some(&d) = chars.peek() {
                    let Some(digit) = d.to_digit(10) else { break };
                    chars.next();
                    pos += 1;
                    value = value
                        .checked_mul(10)
                        .and_then(|v| v.checked_add(u64::from(digit)))
                        .ok_or(LexError::Overflow { pos: start })?;
                }
                Token::Num(value)
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut name = String::from(c);
                while let Some(&d) = chars.peek() {
                    if !(d.is_ascii_alphanumeric() || d == '_') {
                        break;
                    }
                    name.push(d);
                    chars.next();
                    pos += 1;
                }
                Token::Ident(name)
            }
            '-' => {
                if chars.next_if_eq(&'>').is_some() {
                    pos += 1;
                    Token::Arrow
                } else {
                    Token::Op('-')
                }
            }
            '+' | '*' | '/' | '(' | ')' | '=' => Token::Op(c),
            _ => return Err(LexError::Unexpected { ch: c, pos: start }),
        };
        tokens.push(token);
    }
    Ok(tokens)
}
```

Collecting the characters into a `Vec<char>` and working with indices also works. The index is then
the position in characters, and looking ahead is just `chars.get(i)`.

```rust
pub fn tokenize(src: &str) -> Result<Vec<Token>, LexError> {
    let chars: Vec<char> = src.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let start = i;
        let c = chars[i];
        i += 1;
        let token = match c {
            _ if c.is_whitespace() => continue,
            '0'..='9' => {
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let digits: String = chars[start..i].iter().collect();
                let value = digits.parse().map_err(|_| LexError::Overflow { pos: start })?;
                Token::Num(value)
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                Token::Ident(chars[start..i].iter().collect())
            }
            '-' if chars.get(i) == Some(&'>') => {
                i += 1;
                Token::Arrow
            }
            '+' | '-' | '*' | '/' | '(' | ')' | '=' => Token::Op(c),
            _ => return Err(LexError::Unexpected { ch: c, pos: start }),
        };
        tokens.push(token);
    }
    Ok(tokens)
}
```
