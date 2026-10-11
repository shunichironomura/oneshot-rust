A `Validator` holds a list of rules. A rule is a function that takes a string slice and returns
whether the string is acceptable. All the bodies are already written, but the code does not compile.
Change **only the struct definition and the signatures** (the `struct` with its field, and the
`impl` and `fn` lines) so that the test harness (the `tests.rs` tab) compiles and passes. Do not
change the bodies.

```rust
pub struct Validator {
    rules: Vec<Box<dyn Fn(&str) -> bool>>,
}

impl Validator {
    pub fn new() -> Self
    pub fn add(&mut self, rule: impl Fn(&str) -> bool)
    pub fn add_boxed(&mut self, rule: Box<dyn Fn(&str) -> bool>)
    pub fn check(&self, s: &str) -> bool
    pub fn passing(&self, items: &[&str]) -> Vec<&str>
}

pub fn banning(words: &[String]) -> Validator
```

- `add(rule)` adds a closure or an ordinary function as a rule. `add_boxed(rule)` adds a rule that
  is already in a `Box`.
- `check(s)` returns `true` when every rule accepts `s`. A validator without rules accepts every
  string.
- `passing(items)` returns the items that pass `check`, in their original order.
- `banning(words)` returns a validator with a single rule that rejects every string containing one
  of `words` anywhere in it (case matters). If `words` is empty, it accepts every string.

The rules in the harness borrow local variables of the test: a minimum length, a list of reserved
names, a list of banned words, and so on. The harness also checks strings that are created after the
validator, sometimes passes the items to `passing` in a temporary `Vec`, and keeps using the `Vec`
returned by `passing` after the validator, and even the variables its rules borrow, are gone.

### Examples

```rust
let min = 3;
let reserved = vec![String::from("admin"), String::from("root")];
let mut v = Validator::new();
v.add(|s| s.len() >= min);
v.add(|s| !reserved.iter().any(|r| r == s));
v.check("alice") // true
v.check("bo")    // false: too short
v.check("root")  // false: reserved
```

```rust
let words = vec![String::from("spam"), String::from("ad")];
let mut v = banning(&words);
let no_space: Box<dyn Fn(&str) -> bool> = Box::new(|s| !s.contains(' '));
v.add_boxed(no_space);
v.passing(&["hello", "spammer", "bad", "good day"]) // ["hello"]
```

```rust
let items = ["kiwi", "banana", "fig"];
let max = 4;
let mut v = Validator::new();
v.add(|s| s.len() <= max);
let short = v.passing(&items);
drop(v);
// short is ["kiwi", "fig"]
```
