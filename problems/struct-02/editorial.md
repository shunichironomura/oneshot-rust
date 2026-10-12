`Default` is implemented by hand: `#[derive(Default)]` would use the default of each field, which
gives an empty name and `0` retries. Once it exists, `..Default::default()` fills in every field you
do not list, so `with_tags` only states what differs from the default. The one place where you
cannot use it is `default` itself: there, `..Default::default()` calls the same function again and
never returns (rustc warns that the function "cannot return without recursing").

The struct update syntax `..base` is not "copy the rest": like an assignment, it **moves** the
remaining fields out of `base` (and copies the `Copy` ones). In `quiet`, `base` is owned, so moving
its `name` and `tags` into the new value is fine. In `renamed`, `base` is only borrowed.
`Config { name: name.to_string(), ..*base }` would have to move `tags` out of `*base`, and you
cannot move out of a reference: error E0507. `..base.clone()` takes the fields from a fresh copy
instead. (`..base` alone does not compile either, since it is a `&Config`, not a `Config`.)

In `with_tags`, `tags.iter()` yields `&&str` items. `to_string()` works on them, but
`map(String::from)` does not compile, because `String` implements `From<&str>`, not `From<&&str>`.

### Alternatives

`..base.clone()` also clones the old name, only to drop it. Listing `tags` explicitly avoids that:
the fields left for `..*base` are `retries` and `verbose`, which are `Copy`, so nothing is moved out
of the reference.

```rust
pub fn renamed(base: &Config, name: &str) -> Config {
    Config {
        name: name.to_string(),
        tags: base.tags.clone(),
        ..*base
    }
}
```

You can also clone first and change one field of the copy. In the same way, `quiet` can change its
owned argument in place: `mut` on a parameter does not change the signature.

```rust
pub fn renamed(base: &Config, name: &str) -> Config {
    let mut cfg = base.clone();
    cfg.name = name.to_string();
    cfg
}

pub fn quiet(mut base: Config) -> Config {
    base.verbose = false;
    base.retries = 0;
    base
}
```

To use `String::from` in `with_tags`, turn the `&&str` items into `&str` with `copied()` first.

```rust
pub fn with_tags(tags: &[&str]) -> Config {
    Config {
        tags: tags.iter().copied().map(String::from).collect(),
        ..Config::default()
    }
}
```
