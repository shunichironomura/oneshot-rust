Let chains (Rust 2024) let one `if` join `let` patterns and ordinary conditions with `&&`. Each
binding can be used by the conditions after it and by the body, so "starts with `#`, six bytes
follow, all of them hex digits, and they parse" is one flat condition instead of nested `if let`
blocks. The chain may only use `&&`: a `let` cannot be combined with `||`.

The trap is rule 2. Like `str::parse`, `u32::from_str_radix` accepts a leading `+`, so
`"#+12345"` has six characters after the `#` and `u32::from_str_radix("+12345", 16)` returns
`Ok(0x12345)`. "Six characters that parse as hex" is not the same as "six hex digits": check every
byte with `is_ascii_hexdigit` first. After that check `from_str_radix` cannot fail, and putting it
into the chain as `let Ok(value) = ...` avoids an `unwrap`. Use `strip_prefix('#')` rather than
slicing with `&tok[..1]`: a byte range that ends inside a multi-byte character such as `é` panics.

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
            if let Some(digits) = tok.strip_prefix('#')
                && digits.len() == 6
                && digits.bytes().all(|b| b.is_ascii_hexdigit())
                && let Ok(value) = u32::from_str_radix(digits, 16) =>
        {
            Token::Hex(value)
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
