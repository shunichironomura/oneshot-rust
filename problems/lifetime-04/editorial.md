A function that returns a reference must say which argument the reference borrows from. When the
lifetimes are left out, the compiler fills them in with the elision rules: each reference parameter
gets its own lifetime, and if there is exactly one, the return value gets it too (in a method, the
return value gets the lifetime of `&self`). `first_word` has a single reference parameter, so it
already compiles and needs no change. `strip_tag` and `pick` have two (`first` is not a reference),
so the compiler refuses to guess and reports E0106 (missing lifetime specifier).

`pick` may return either `a` or `b`, so the result is valid only while **both** are: give both
parameters and the return type the same lifetime `'a`. Arguments that live longer are fine, because
the compiler chooses an `'a` short enough to fit all of them.

`strip_tag` is the trap. Putting `'a` on every reference, as for `pick` (and as the compiler's help
message suggests), compiles on its own, but it claims that the result may borrow from `tag` as well.
The caller must then keep `tag` alive as long as the result, and the harness, which builds the tag in
a temporary `String` and drops it before using the result, fails with "`tag` does not live long
enough" (E0597). The body only ever returns slices of `line`, so only `line` shares the lifetime
of the result, and `tag` keeps its own, elided lifetime.

### Alternatives

Naming the second lifetime makes it explicit that `tag` is unrelated to the result. `first_word` can
also be written with the lifetime that elision fills in for you.

```rust
pub fn strip_tag<'a, 'b>(line: &'a str, tag: &'b str) -> &'a str {
    match line.strip_prefix(tag) {
        Some(rest) => rest.trim_start(),
        None => line,
    }
}

pub fn first_word<'a>(s: &'a str) -> &'a str {
    s.split_whitespace().next().unwrap_or("")
}
```

`pick` can also take two lifetimes, as long as the lifetime of `b` is declared to last at least as
long as the result's: `'b: 'a` reads "`'b` outlives `'a`", which lets `b` be returned as a `&'a str`.

```rust
pub fn pick<'a, 'b: 'a>(a: &'a str, b: &'b str, first: bool) -> &'a str {
    if first { a } else { b }
}
```
