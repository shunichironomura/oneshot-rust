Write a declarative macro `json!` that builds a `Json` value from JSON-like syntax. The `Json` type
and its `From` impls are given (they are already in the editor). The macro is in the editor as a
placeholder rule that accepts any input, followed by a `pub(crate) use` line that lets other
modules call it by path. Replace the rule and keep the `pub(crate) use` line.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl From<bool> for Json { /* Json::Bool */ }
impl From<i32> for Json { /* Json::Num */ }
impl From<f64> for Json { /* Json::Num */ }
impl From<&str> for Json { /* Json::Str */ }
impl From<String> for Json { /* Json::Str */ }

macro_rules! json { /* ... */ }
pub(crate) use json;
```

`json!(v)` takes one value `v` and evaluates to the matching `Json`. A value is one of:

- `null`, which is `Json::Null`.
- `true` or `false`, which is `Json::Bool`.
- A number literal: an integer literal such as `3` (an `i32`), or a floating-point literal such as
  `2.5` (an `f64`). It is `Json::Num`, so `json!(3)` is `Json::Num(3.0)`.
- A string literal such as `"s"`, which is `Json::Str`.
- An array `[v1, v2, ...]` of values, which is `Json::Arr` with the elements in the written order.
- An object `{ "k1": v1, "k2": v2, ... }`, which is `Json::Obj` with the pairs in the written
  order. Every key is a string literal. When a key appears more than once, every pair is kept.
- Any other Rust expression, wrapped in parentheses, such as `(x - 7)`, `(-1)` or
  `(name.clone())`. Its type is `bool`, `i32`, `f64`, `&str`, `String` or `Json`, and the value is
  `Json::from(expression)` (for `Json`, this is the value itself). Expressions that are not one of
  the literals above must be parenthesized, negative numbers included.

Arrays and objects can be nested inside each other, can be empty (`[]`, `{}`), and may have a
trailing comma after the last element (`[1, 2,]`, `{ "a": 1, }`). Every parenthesized expression is
evaluated exactly once, in the order it is written.

The tests call the macro by its path, for example `crate::user::json!(null)`, from another module
that has no `use` declarations for `json` or `Json`.

### Examples

```rust
crate::user::json!({ "a": 1, "b": [true, null, "s"], "c": {} })
// Json::Obj(vec![
//     ("a".to_string(), Json::Num(1.0)),
//     ("b".to_string(), Json::Arr(vec![Json::Bool(true), Json::Null, Json::Str("s".to_string())])),
//     ("c".to_string(), Json::Obj(vec![])),
// ])

let x: i32 = 4;
crate::user::json!([(x - 7), "x"]) // Json::Arr(vec![Json::Num(-3.0), Json::Str("x".to_string())])

crate::user::json!(null) // Json::Null
```
