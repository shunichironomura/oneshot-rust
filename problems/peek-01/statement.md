Write the tokenizer of a tiny expression language: `tokenize` splits the source text into tokens,
or reports the first error.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Num(u64),
    Op(char),
    Arrow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexError {
    Unexpected { ch: char, pos: usize },
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, LexError>
```

Both types are already in the editor. The text is read from left to right, and each token is made
as long as possible:

- A run of ASCII digits (`0` to `9`) is a `Num` holding its decimal value. Leading zeros are allowed
  and do not change the value: `007` is `Num(7)`. The value always fits in a `u64`.
- `->` (a `-` immediately followed by `>`) is `Arrow`.
- Each of `+ - * / ( ) =` on its own is an `Op` holding that character.
- Whitespace separates tokens and produces no token. Whitespace is every character for which
  `char::is_whitespace` returns `true`, including non-ASCII spaces such as U+3000.
- Any other character is an error, `LexError::Unexpected` holding that character and its position.
  This includes letters, a `>` that is not part of `->`, and non-ASCII characters other than
  whitespace, even digits such as `٣` (U+0663).
- `pos` is a 0-based position counted in characters (`char`s), not bytes.
- When the text has several errors, return only the first one (the one with the smallest `pos`).

### Examples

Results are shown as `{:?}` prints them.

```rust
tokenize("1 -> 42+3") // Ok([Num(1), Arrow, Num(42), Op('+'), Num(3)])
tokenize("5-2")       // Ok([Num(5), Op('-'), Num(2)])
tokenize("007*(1)")   // Ok([Num(7), Op('*'), Op('('), Num(1), Op(')')])
tokenize("1 >= 1")    // Err(Unexpected { ch: '>', pos: 2 })
```
