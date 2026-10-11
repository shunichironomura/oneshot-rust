`Borrow<str>` is what lets `set.contains("alice")` and `map.get("bob")` take a `&str`, but it comes
with a contract: `Eq`, `Ord` and `Hash` must give the same answers for a `Username` and for its
`borrow()`. A hash set looks up `"alice"` by hashing the `&str` with the `Hash` of `str`, while each
username in it was placed by the `Hash` of `Username`. If the two hashes differ, `contains` looks in
the wrong place and usually answers `false`: no compile error, no panic, not even UB, just wrong
results. A `BTreeMap` has the same problem with order: `get("bob")` compares `"bob"` with the keys
using the `Ord` of `str`, in a tree that was arranged with the `Ord` of `Username`.

The easiest way to keep the contract is to derive. On a struct with a single field, the derived
`PartialEq`, `Eq`, `PartialOrd`, `Ord` and `Hash` only call those of the field, and `String`
compares and hashes exactly like `str`. Implementing them by hand works too, as long as each one
delegates to `self.0` (see the alternatives). Code that looks just as reasonable breaks lookups:
`state.write(self.0.as_bytes())` misses the extra `0xff` byte that the `Hash` of `str` writes after
the bytes, `self.0.as_bytes().hash(state)` hashes the length of the slice first, and ordering by
length first disagrees with the order of `str`.

`Deref<Target = str>` gives the rest of the "works like a `&str`" behavior. In `deref`, `&self.0`
is a `&String`, which deref coercion turns into the `&str` of the signature. A method call such as
`u.len()` finds no `len` on `Username`, so it auto-derefs to `str`. And `&u` coerces to `&str`
wherever a `&str` is expected, such as an argument of a function that takes a `&str`, or
`let s: &str = &u;`. Coercion does not happen for a generic parameter such as `S: AsRef<str>`,
which is why `AsRef<str>` is implemented as well. There is no `DerefMut`, so nobody can change the
name of a `Username` and make it invalid.

In `PartialEq<&str>`, the right-hand side is a `&str`, so `other` is a `&&str`. `self.0 == other`
does not compile, since there is no `String: PartialEq<&&str>`. Dereference once:
`self.0 == *other` compares a `String` with a `&str`.

In `parse`, count characters with `chars().count()`: `len()` counts bytes, so `"aé"` (2 characters,
3 bytes) would not be `TooShort`. Test characters with the ASCII-only range patterns `'a'..='z'` and
`'0'..='9'` (or `is_ascii_lowercase` and `is_ascii_digit`): `is_lowercase` and `is_alphanumeric`
also accept letters such as `é` and digits such as `٣`.

`fmt::Display::fmt(&self.0, f)` forwards to the `Display` of `str`, which also honors width and
alignment such as `{:>10}`. `f.write_str(&self.0)` prints the same name but ignores them (the tests
do not use them).

### Alternatives

The comparison and hash impls of the template can be filled in by hand. Each one delegates to the
`String`, so they agree with `str` just like the derived ones. Keep `#[derive(Debug, Clone)]` and
the imports of the template (`Ordering`, `Hash` and `Hasher`).

```rust
impl PartialEq for Username {
    fn eq(&self, other: &Username) -> bool {
        self.0 == other.0
    }
}

impl Eq for Username {}

impl PartialOrd for Username {
    fn partial_cmp(&self, other: &Username) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Username {
    fn cmp(&self, other: &Username) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Hash for Username {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
```

`parse` can check every character in one loop. The first character must be a lowercase letter, and
once it is, it also passes the check for the other characters.

```rust
pub fn parse(s: &str) -> Result<Username, NameError> {
    let name = s.trim();
    match name.chars().count() {
        0..=2 => return Err(NameError::TooShort),
        3..=16 => {}
        _ => return Err(NameError::TooLong),
    }
    for (i, c) in name.chars().enumerate() {
        if i == 0 && !c.is_ascii_lowercase() {
            return Err(NameError::BadStart);
        }
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
            return Err(NameError::BadChar(c));
        }
    }
    Ok(Username(name.to_string()))
}
```

The one-line bodies can also go through `Deref`. `**self` and `**other` are both `str`, and
`self` coerces to the `&str` that `write_str` takes.

```rust
impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self)
    }
}

impl PartialEq<&str> for Username {
    fn eq(&self, other: &&str) -> bool {
        **self == **other
    }
}
```
