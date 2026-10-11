A sign-up form accepts usernames such as `alice_1`. Make `Username`, a newtype around `String`,
that can only hold a valid name and yet can be used almost anywhere a `&str` can. The types and the
`impl` blocks are already in the editor; implement them.

```rust
#[derive(Debug, Clone)]
pub struct Username(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameError {
    TooShort,
    TooLong,
    BadStart,
    BadChar(char),
}

impl Username {
    pub fn parse(s: &str) -> Result<Username, NameError>;
}

impl Deref for Username { type Target = str; /* to implement */ }
impl AsRef<str> for Username { /* to implement */ }
impl Borrow<str> for Username { /* to implement */ }
impl fmt::Display for Username { /* to implement */ }
impl PartialEq<&str> for Username { /* to implement */ }
impl PartialEq for Username { /* to implement */ }
impl Eq for Username {}
impl PartialOrd for Username { /* to implement */ }
impl Ord for Username { /* to implement */ }
impl Hash for Username { /* to implement */ }
```

`Username::parse(s)` first removes whitespace from both ends of `s`, as `str::trim` does. What
remains is the name. It goes through the following checks in this order, and the first check that
fails gives the error:

1. The name has at least 3 characters (`char`s). Otherwise: `TooShort`.
2. The name has at most 16 characters. Otherwise: `TooLong`.
3. The first character is an ASCII lowercase letter (`a` to `z`). Otherwise: `BadStart`.
4. Each of the other characters is an ASCII lowercase letter, an ASCII digit (`0` to `9`) or `_`.
   Otherwise: `BadChar(c)`, where `c` is the first character that is not.

If every check passes, `parse` returns `Ok` of a `Username` that holds the name.

A `Username` stands for its name everywhere:

- It dereferences to its name, a `str`. So `str` methods work on it (`u.len()`,
  `u.starts_with("al")`, `&u[1..3]`, ...), and `&u` can be passed where a `&str` is expected.
- `as_ref()` and `borrow()` return the name, and `Display` prints it. (The tests also join a slice
  of usernames with `join(", ")`.)
- `u == "alice"`, which compares a `Username` with a `&str`, is true exactly when the name is
  `alice`.
- Two usernames are equal when their names are equal, and usernames are ordered like their names
  (as `str`s): for example, `al_x` < `alice` < `bob`.
- The tests put usernames in `HashSet`s, `HashMap`s, `BTreeSet`s and `BTreeMap`s and query them
  with a `&str`: `set.contains("alice")`, `map.get("bob")`, `map.remove("carol")` and the like
  must find the username with that name.

### Examples

```rust
let u = Username::parse("  alice_1 ").unwrap();
u.len()              // 7
u.starts_with("al")  // true
u == "alice_1"       // true
format!("<{u}>")     // "<alice_1>"

Username::parse("Al")        // Err(NameError::TooShort)
Username::parse("9lives")    // Err(NameError::BadStart)
Username::parse("bad-name")  // Err(NameError::BadChar('-'))

let mut set = HashSet::new();
set.insert(Username::parse("alice").unwrap());
set.insert(Username::parse("bob").unwrap());
set.contains("bob")          // true
```
