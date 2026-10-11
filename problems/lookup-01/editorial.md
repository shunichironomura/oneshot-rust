The wrappers need the same shape as `HashMap::get` itself:

```rust
pub fn get<Q>(&self, k: &Q) -> Option<&V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
```

`K: Borrow<Q>` says that a `&K` can be viewed as a `&Q`: `String` implements `Borrow<str>`,
`Vec<T>` implements `Borrow<[T]>`, and every type implements `Borrow<Self>`, which is why `&String`,
`&Vec<u8>` and `&u32` keep working. `Borrow` (unlike `AsRef`) also promises that the borrowed form
hashes and compares exactly like the owned key, so the map can hash a `&str` and find the `String`.
`Q` belongs to each method, not to the `impl`, so that every call can choose its own form of the key,
and the bound on `K` mentions `Q`, so it goes in the method's `where` clause. The other two parts of
the bound are easy to forget:

- `Q: ?Sized`: type parameters are `Sized` by default, but `str` and `[u8]` are not, so without it
  `get("apple")` fails with "the size for values of type `str` cannot be known at compilation time".
- `Q: Hash + Eq`: `HashMap::get` hashes and compares the query, so it requires these traits on `Q`.
  The `K: Hash + Eq` of the `impl` does not provide them.

`get_or` is the trap. With elision, a method's result gets the lifetime of `&self`, so the template
promises a result borrowed from `self` alone, and the body, which may return `default`, fails with
"lifetime may not live long enough". The compiler's suggestion, a named lifetime on `default` and
the result only, fails the other way round: then the reference from `self.map.get(key)` is the one
that does not live long enough. The result may come from either place, so `&self`, `default` and the
result share one lifetime `'a`. A registry that lives longer than `default` is fine, because the
compiler picks an `'a` short enough for both.

Keep the key out of `'a`. `key: &'a Q` compiles on its own, but it says that the result may borrow
from the key as well. The harness, which looks up with a temporary `String` that is dropped before
the result is used, then fails with "temporary value dropped while borrowed". `get`, `contains` and
`remove` need no lifetimes at all: elision ties the result to `&self` and gives `key` its own
lifetime, which is exactly right.

### Alternatives

Bounds on `Q` alone can go in the angle brackets. `K: Borrow<Q>` constrains `K`, so it stays in the
`where` clause. `contains` and `remove` change in the same way.

```rust
pub fn get<Q: Hash + Eq + ?Sized>(&self, key: &Q) -> Option<&V>
where
    K: Borrow<Q>,
{
    self.map.get(key)
}

pub fn get_or<'a, Q: Hash + Eq + ?Sized>(&'a self, key: &Q, default: &'a V) -> &'a V
where
    K: Borrow<Q>,
{
    self.map.get(key).unwrap_or(default)
}
```

`get_or` can also give `self` a lifetime of its own, as long as it outlives the result's: `'s: 'a`
reads "`'s` outlives `'a`", which lets a `&'s V` from the map be returned as a `&'a V`.

```rust
pub fn get_or<'s, 'a, Q>(&'s self, key: &Q, default: &'a V) -> &'a V
where
    's: 'a,
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
{
    self.map.get(key).unwrap_or(default)
}
```
