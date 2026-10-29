A function that returns a reference must say which argument the reference borrows from. When the
lifetimes are left out, the compiler fills them in with the elision rules: each reference parameter
gets its own lifetime, and if there is exactly one, the return value gets it too (in a method, the
return value gets the lifetime of `&self`). `strip_tag` has two reference parameters, so the
compiler refuses to guess and reports E0106 (missing lifetime specifier).

The trap is the obvious fix. Putting `'a` on every reference (as the compiler's help message
suggests) compiles on its own, but it claims that the result may borrow from `tag` as well. The
caller must then keep `tag` alive as long as the result, and the harness, which builds the tag in a
temporary `String` and drops it before using the result, fails with "`tag` does not live long
enough" (E0597). The body only ever returns slices of `line`, so only `line` shares the lifetime of
the result, and `tag` keeps its own, elided lifetime.

### Alternatives

Naming the second lifetime makes it explicit that `tag` is unrelated to the result.

```rust
pub fn strip_tag<'a, 'b>(line: &'a str, tag: &'b str) -> &'a str {
    match line.strip_prefix(tag) {
        Some(rest) => rest.trim_start(),
        None => line,
    }
}
```
