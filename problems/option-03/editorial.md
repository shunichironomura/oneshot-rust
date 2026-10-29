`Option::take_if(f)` (Rust 1.80) takes `&mut self`. It calls `f` with a `&mut T` and takes the
value out, leaving `None`, only if `f` returns `true`. The tempting alternatives go wrong:

- `slot.filter(...)`, `slot.is_some_and(...)`, `slot.map(...)` and `slot.unwrap()` take the
  `Option` **by value**. Through `slot: &mut Option<Session>`, that is a move out of a reference
  (E0507). These methods are for an `Option` that the function owns and may consume.
- `slot.take().filter(...)` compiles, but `take` always empties the slot first, so a session that
  is still valid is dropped instead of being put back.
- `slot.as_ref().filter(...).cloned()` returns a copy and leaves the expired session in the slot.

### Alternatives

A plain `match` works just as well. `session` borrows from `slot`, but only the guard uses it, so
the borrow is over before `slot.take()` runs.

```rust
pub fn evict_expired(slot: &mut Option<Session>, now: u64) -> Option<Session> {
    match slot {
        Some(session) if session.expires_at <= now => slot.take(),
        _ => None,
    }
}
```

`is_some_and` can test the slot too, if you look at it through `as_ref()`, which gives an
`Option<&Session>` without moving anything.

```rust
pub fn evict_expired(slot: &mut Option<Session>, now: u64) -> Option<Session> {
    if slot.as_ref().is_some_and(|session| session.expires_at <= now) {
        slot.take()
    } else {
        None
    }
}
```
