A small web server keeps track of logged-in users with the `Session` struct below, and each slot of
its session table may hold one session. Implement the function `evict_expired`. `Session` is
already in the editor.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub user: String,
    pub expires_at: u64,
    pub admin: bool,
}

pub fn evict_expired(slot: &mut Option<Session>, now: u64) -> Option<Session>
```

Times are in seconds. A session is valid while `now < expires_at`, and it has **expired** once
`expires_at <= now`. `now` can be any `u64`, before or after `expires_at`.

- If the slot holds an expired session, `evict_expired` takes the session out of the slot (the slot
  becomes `None`) and returns it.
- Otherwise (the slot is empty, or its session is still valid) it returns `None` and leaves the slot
  unchanged.

### Examples

```rust
let root = Session { user: "root".to_string(), expires_at: 100, admin: true };
let mut slot = Some(root.clone());
evict_expired(&mut slot, 100) // Some(root): the session is taken out, and slot is now None
```
