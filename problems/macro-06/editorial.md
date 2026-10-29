Matching with `$t:tt` takes a whole group such as `(b c)` as one token tree. The solution peels off
one token tree at a time and adds 1 for it. The shorter `0usize $(+ 1)*` is rejected ("attempted to
repeat an expression containing no syntax variables"): a repetition in the transcriber must contain
a metavariable from the matcher.

`count!` must be a constant expression, so `vec![...].len()` does not work: `vec!` allocates on the
heap, which a constant cannot do. `1usize + $crate::user::count!($($tail)*)` is constant. Each token
tree adds one level of recursion, and the default recursion limit is 128, which is plenty here.

Paths in the expansion are resolved where the macro is called, and macro names behave the same way.
A `macro_rules!` macro can be called by its bare name only below its definition (in the same module
or its submodules), or where it is imported. When the tests call `crate::user::count!` by path, a
recursive `count!(...)` in the expansion is looked up in the tests module and is not found ("cannot
find macro `count` in this scope"). `$crate` always names the crate that defines the macro, so
`$crate::user::count!` works from anywhere.

### Alternatives

`count!` without recursion: turn each token tree into a string with `stringify!` and take the length
of the slice. `<[&str]>::len` is a `const fn`. Writing `[...].len()` instead fails for `count!()`,
because the element type of an empty array is unknown.

```rust
macro_rules! count {
    ($($t:tt)*) => {
        <[&str]>::len(&[$(stringify!($t)),*])
    };
}
pub(crate) use count;
```
