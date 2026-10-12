Let chains (Rust 2024) let one `if` join `let` patterns and ordinary conditions with `&&`. Each
binding can be used by the conditions after it and by the body, so "starts with `'`, the rest ends
with `'`, and what is between has no `'`" is one flat condition instead of nested `if let` blocks.
If any part of the chain fails, the whole condition is false and `classify` goes on to the next
rule, which is exactly "the first rule that applies": `'it's:x'` is not quoted, so it becomes a
`Pair`. The chain may only use `&&`: a `let` cannot be combined with `||`.

The trap is rule 1. `tok.starts_with('\'') && tok.ends_with('\'')` is also true for the token `'`
on its own, where the opening and the closing quote would be the same character. Then
`&tok[1..tok.len() - 1]` is `&tok[1..0]`, and a range that ends before it starts panics ("byte
range starts at 1 but ends at 0"). `tok.trim_matches('\'')` does not panic, but it removes **every**
`'` at both ends, so `'` and `'''` would both become `Quoted("")`. Strip the opening quote first and
look for the closing one only in what is left: `strip_prefix` and `strip_suffix` return `None`
instead of panicking, and in a let chain the second `let` can work on the binding of the first.
For the same reason, `strip_prefix('\'')` is safer than comparing `&tok[..1]`, which panics on the
empty string and on a token that starts with a multi-byte character such as `é`.

`split_once(':')` splits at the first `:` and returns `None` when there is none, which is exactly
rule 2. Checking `!before.is_empty() && !after.is_empty()` in the same chain sends `":b"`, `"a:"`
and `":"` on to `Word`.

### Alternatives

Slicing is fine once the length rules out the lone quote: a token of at least two bytes that starts
and ends with `'` has two different quotes, and `'` is one byte, so `1` and `tok.len() - 1` are
character boundaries. Counting the quotes covers the "no `'` inside" part.

```rust
pub fn classify(tok: &str) -> Token {
    if tok.len() >= 2
        && tok.starts_with('\'')
        && tok.ends_with('\'')
        && tok.matches('\'').count() == 2
    {
        return Token::Quoted(tok[1..tok.len() - 1].to_string());
    }
    if let Some((before, after)) = tok.split_once(':')
        && !before.is_empty()
        && !after.is_empty()
    {
        return Token::Pair(before.to_string(), after.to_string());
    }
    Token::Word(tok.to_string())
}
```
