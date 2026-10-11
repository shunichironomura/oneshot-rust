The `Describe` trait turns a value into a short text. The trait and its implementations are already
in the editor, and all the method bodies are correct. But some of the `impl` lines were written for
the one type the author needed first, so the test harness (the `tests.rs` tab) does not compile.
Change **only the `impl` lines** (the part of each `impl` block before its `{`) so that the harness
compiles and passes. Do not change the trait or the method bodies, and do not add or remove `impl`
blocks.

```rust
pub trait Describe {
    fn describe(&self) -> String;
}

impl Describe for i32 { /* ... */ }
impl Describe for bool { /* ... */ }
impl Describe for str { /* ... */ }
impl Describe for String { /* ... */ }
impl Describe for [i32] { /* ... */ }
impl Describe for Vec<i32> { /* ... */ }
impl Describe for Option<i32> { /* ... */ }
impl Describe for (i32, i32) { /* ... */ }
impl<T: Describe + ?Sized> Describe for &T { /* ... */ }
```

The bodies describe values as follows.

- An `i32` is written in decimal, and a `bool` as `yes` or `no`.
- A string (`str` or `String`) is written the way `{:?}` formats it: in double quotes, with `"`, `\`
  and control characters such as a line break escaped.
- A slice or a `Vec` is written as `[`, the descriptions of its elements separated by a comma and a
  space, and `]`. An empty one is `[]`.
- `Some(x)` is written as `some(`, the description of `x`, and `)`. `None` is `none`.
- A pair `(a, b)` is written as `(`, the description of `a`, a comma and a space, the description of
  `b`, and `)`.
- A reference is described like the value it points to.

The elements of a slice or a `Vec`, the value in an `Option`, and the two parts of a pair can be of
any type that implements `Describe`, and the two parts of a pair may have different types. The
harness nests these types, describes references at any depth, and uses string slices (`&str`),
references to slices (`&[i32]`, `&[bool]`), trait objects (`&dyn Describe`) and a type of its own,
`Celsius`, as elements.

### Examples

```rust
"hi".describe()                     // "\"hi\""
vec![Some(1), None].describe()      // "[some(1), none]"
vec![&5, &6].describe()             // "[5, 6]"
(true, vec!["a", "b"]).describe()   // "(yes, [\"a\", \"b\"])"
Some(String::from("x")).describe()  // "some(\"x\")"
```
