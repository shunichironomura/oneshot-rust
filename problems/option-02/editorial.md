With the right helper methods of `Option` and `bool`, each function takes one to three lines. But
each helper has a detail that is easy to get wrong when you write it from memory.

- `is_admin`: `Option::is_some_and(f)` is `true` only for a `Some(x)` where `f(x)` is `true`.
- `within_limit`: `Option::is_none_or(f)` (Rust 1.82) is its counterpart: `true` for `None`, and
  `f(x)` for `Some(x)`. `attempts < limit.unwrap_or(u32::MAX)` looks equivalent, but it is wrong
  for `attempts == u32::MAX`: no `u32` is larger than every other `u32`.
- `time_left`: like any function argument, the value passed to `bool::then_some(v)` is computed
  **before** the call, whether the condition is true or not. So
  `(now < s.expires_at).then_some(s.expires_at - now)` subtracts even for an expired session, and as
  soon as `now > expires_at` the subtraction overflows, which panics in debug builds and under Miri.
  `bool::then` takes a closure and runs it only when the condition is true. `checked_sub` avoids the
  overflow, but on its own it returns `Some(0)` when `now == expires_at`.
- `evict_expired`: `Option::take_if(f)` (Rust 1.80) takes `&mut self`. It calls `f` with a `&mut T`
  and takes the value out, leaving `None`, only if `f` returns `true`. The tempting alternatives go
  wrong:
  - `slot.filter(...)`, `slot.is_some_and(...)`, `slot.map(...)` and `slot.unwrap()` take the
    `Option` **by value**. Through `slot: &mut Option<Session>`, that is a move out of a reference
    (E0507). The same methods were fine in `is_admin` and `within_limit` because `Option<&Session>`
    and `Option<u32>` are `Copy`.
  - `slot.take().filter(...)` compiles, but `take` always empties the slot first, so a session that
    is still valid is dropped instead of being put back.
  - `slot.as_ref().filter(...).cloned()` returns a copy and leaves the expired session in the slot.
- `authorize`: `Option::ok_or(e)` turns `None` into `Err(e)`, and `bool::ok_or(e)` (Rust 1.98) turns
  `false` into `Err(e)` and `true` into `Ok(())`. With `?`, each check becomes one line, and the
  order of the lines is the order of the checks. The last check needs no `?`: its
  `Result<(), AuthError>` already is the return value.

### Alternatives

Plain `match` and `if` work just as well. In `evict_expired`, `session` borrows from `slot`, but
only the guard uses it, so the borrow is over before `slot.take()` runs.

```rust
pub fn is_admin(s: Option<&Session>) -> bool {
    match s {
        Some(session) => session.admin,
        None => false,
    }
}

pub fn within_limit(limit: Option<u32>, attempts: u32) -> bool {
    match limit {
        Some(max) => attempts < max,
        None => true,
    }
}

pub fn time_left(s: &Session, now: u64) -> Option<u64> {
    if now < s.expires_at { Some(s.expires_at - now) } else { None }
}

pub fn evict_expired(slot: &mut Option<Session>, now: u64) -> Option<Session> {
    match slot {
        Some(session) if session.expires_at <= now => slot.take(),
        _ => None,
    }
}

pub fn authorize(s: Option<&Session>, now: u64) -> Result<(), AuthError> {
    let Some(s) = s else {
        return Err(AuthError::NoSession);
    };
    if s.expires_at <= now {
        Err(AuthError::Expired)
    } else if !s.admin {
        Err(AuthError::NotAdmin)
    } else {
        Ok(())
    }
}
```

`checked_sub` works for `time_left` once a zero result is filtered out, and `map_or` was the usual
way to write `within_limit` before `is_none_or` existed.

```rust
pub fn time_left(s: &Session, now: u64) -> Option<u64> {
    s.expires_at.checked_sub(now).filter(|&left| left > 0)
}

pub fn within_limit(limit: Option<u32>, attempts: u32) -> bool {
    limit.map_or(true, |max| attempts < max)
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
