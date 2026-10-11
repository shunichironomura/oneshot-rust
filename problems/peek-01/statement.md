Write the tokenizer of a tiny expression language: `tokenize` splits the source text into tokens,
or reports the first error.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Num(u64),
    Ident(String),
    Op(char),
    Arrow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexError {
    Unexpected { ch: char, pos: usize },
    Overflow { pos: usize },
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, LexError>
```

Both types are already in the editor. The text is read from left to right, and each token is made
as long as possible:

- A run of ASCII digits (`0` to `9`) is a `Num` holding its decimal value. Leading zeros are allowed
  and do not change the value: `007` is `Num(7)`. If the value is greater than `u64::MAX`, the
  result is `LexError::Overflow`.
- An identifier starts with an ASCII letter or `_`, followed by zero or more ASCII letters, ASCII
  digits and `_` (so `_` alone is one too). It becomes an `Ident` holding its text. A digit cannot
  start one: `3x` is `Num(3)` followed by `Ident("x")`.
- `->` (a `-` immediately followed by `>`) is `Arrow`.
- Each of `+ - * / ( ) =` on its own is an `Op` holding that character.
- Whitespace separates tokens and produces no token. Whitespace is every character for which
  `char::is_whitespace` returns `true`, including non-ASCII spaces such as U+3000.
- Any other character is an error, `LexError::Unexpected` holding that character. This includes a
  `>` that is not part of `->`, and non-ASCII characters other than whitespace, even letters and
  digits such as `é`.
- `pos` is a 0-based position counted in characters (`char`s), not bytes. For `Unexpected` it is the
  position of the character, for `Overflow` the position of the first digit of the number.
- When the text has several errors, return only the first one (the one with the smallest `pos`).

### Examples

Results are shown as `{:?}` prints them.

```rust
tokenize("x1 -> 42+y") // Ok([Ident("x1"), Arrow, Num(42), Op('+'), Ident("y")])
tokenize("a-b")        // Ok([Ident("a"), Op('-'), Ident("b")])
tokenize("3x")         // Ok([Num(3), Ident("x")])
tokenize("a >= 1")     // Err(Unexpected { ch: '>', pos: 2 })
```
