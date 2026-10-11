Let chains (Rust 2024) let one `if` join `let` patterns and ordinary conditions with `&&`. Each
binding can be used by the conditions after it and by the body, so "starts with `'`, the rest ends
with `'`, and what is between has no `'`" is one flat condition instead of nested `if let` blocks.
If any part of the chain fails, the whole condition is false and `classify` goes on to the next
rule, which is exactly "the first rule that applies": `'it's:x'` is not quoted, so it becomes a
`Pair`. The chain may only use `&&`: a `let` cannot be combined with `||`.

The trap is rule 2. `tok.starts_with('\'') && tok.ends_with('\'')` is also true for the token `'`
on its own, where the opening and the closing quote would be the same character. Then
`&tok[1..tok.len() - 1]` is `&tok[1..0]`, and a range that ends before it starts panics ("byte
range starts at 1 but ends at 0"). `tok.trim_matches('\'')` does not panic, but it removes **every**
`'` at both ends, so `'` and `'''` would both become `Quoted("")`. Strip the opening quote first and
look for the closing one only in what is left: `strip_prefix` and `strip_suffix` return `None`
instead of panicking, and in a let chain the second `let` can work on the binding of the first.
For the same reason, `strip_prefix('\'')` is safer than comparing `&tok[..1]`, which panics on the
empty string and on a token that starts with a multi-byte character such as `é`.

Exclusive range patterns such as `0..10` (stable since Rust 1.80) match the rules as written. The
compiler checks that integer ranges cover every value, so `0..10`, `10..1000` and `1000..` need no
`_` arm. A last arm of `1000..u64::MAX` would not compile: an exclusive range leaves out `u64::MAX`
itself.

`split_once(':')` splits at the first `:` and returns `None` when there is none, which is exactly
rule 3. Checking `!before.is_empty() && !after.is_empty()` in the same chain sends `":b"`, `"a:"`
and `":"` on to `Word`.

On a `u8`, `a + b` panics on overflow in debug builds (and under Miri) and wraps around silently in
release builds. `checked_add` returns `None` instead. The slice pattern `[a, b, ..]` matches only
slices with at least two elements, so no index can be out of bounds.

### Alternatives

One `match` on the result of `parse`, with binding range patterns and `if let` guards (stable since
Rust 1.95). A guard can itself be a let chain, and its bindings can be used in the arm. The arms are
tried from top to bottom, so `n @ ..1000` only sees numbers of 10 or more.

```rust
pub fn classify(tok: &str) -> Token {
    match tok.parse::<u64>() {
        Ok(n @ ..10) => Token::Tiny(n),
        Ok(n @ ..1000) => Token::Small(n),
        Ok(n) => Token::Large(n),
        Err(_)
            if let Some(rest) = tok.strip_prefix('\'')
                && let Some(text) = rest.strip_suffix('\'')
                && !text.contains('\'') =>
        {
            Token::Quoted(text.to_string())
        }
        Err(_)
            if let Some((before, after)) = tok.split_once(':')
                && !before.is_empty()
                && !after.is_empty() =>
        {
            Token::Pair(before.to_string(), after.to_string())
        }
        Err(_) => Token::Word(tok.to_string()),
    }
}
```

Slicing is fine once the length rules out the lone quote: a token of at least two bytes that starts
and ends with `'` has two different quotes, and `'` is one byte, so `1` and `tok.len() - 1` are
character boundaries. Counting the quotes covers the "no `'` inside" part.

```rust
pub fn classify(tok: &str) -> Token {
    if let Ok(n) = tok.parse::<u64>() {
        return match n {
            0..10 => Token::Tiny(n),
            10..1000 => Token::Small(n),
            1000.. => Token::Large(n),
        };
    }
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

`sum_first_two` can take the two elements apart with let-else and add them as `u16`, where the sum
cannot overflow, then convert back.

```rust
pub fn sum_first_two(nums: &[u8]) -> Option<u8> {
    let [a, b, ..] = *nums else {
        return None;
    };
    u8::try_from(u16::from(a) + u16::from(b)).ok()
}
```

Or with `?` on `Option`:

```rust
pub fn sum_first_two(nums: &[u8]) -> Option<u8> {
    nums.first()?.checked_add(*nums.get(1)?)
}
```
