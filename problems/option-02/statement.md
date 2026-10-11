A small web server keeps track of logged-in users with the `Session` struct below. Write five
helper functions for it. `Session` and `AuthError` are already in the editor.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub user: String,
    pub expires_at: u64,
    pub admin: bool,
}

#[derive(Debug, PartialEq)]
pub enum AuthError {
    NoSession,
    Expired,
    NotAdmin,
}

pub fn is_admin(s: Option<&Session>) -> bool
pub fn within_limit(limit: Option<u32>, attempts: u32) -> bool
pub fn time_left(s: &Session, now: u64) -> Option<u64>
pub fn evict_expired(slot: &mut Option<Session>, now: u64) -> Option<Session>
pub fn authorize(s: Option<&Session>, now: u64) -> Result<(), AuthError>
```

Times are in seconds. A session is valid while `now < expires_at`, and it has **expired** once
`expires_at <= now`. `now` can be any `u64`, before or after `expires_at`.

- `is_admin` returns `true` if there is a session and its `admin` flag is set, and `false`
  otherwise. It does not look at the expiry time.
- `within_limit` tells whether a user who has failed to log in `attempts` times may try again.
  `None` means there is no limit, and the result is always `true`. `Some(n)` allows `n` attempts in
  total, so the result is `true` exactly when `attempts < n`.
- `time_left` returns the number of seconds the session has left, `Some(expires_at - now)`, or
  `None` if the session has expired.
- `evict_expired` cleans up a slot that may hold a session. If the slot holds an expired session,
  it takes the session out of the slot (the slot becomes `None`) and returns it. Otherwise (the slot
  is empty, or its session is still valid) it returns `None` and leaves the slot unchanged.
- `authorize` lets a request through only with a valid admin session. It runs these checks in this
  order and returns the error of the first one that fails:
  1. there is a session, or else `Err(AuthError::NoSession)`;
  2. the session has not expired, or else `Err(AuthError::Expired)`;
  3. the session is an admin session, or else `Err(AuthError::NotAdmin)`.

  If every check passes, it returns `Ok(())`.

### Examples

```rust
let root = Session { user: "root".to_string(), expires_at: 100, admin: true };
let ann = Session { user: "ann".to_string(), expires_at: 100, admin: false };

is_admin(Some(&root))         // true
is_admin(None)                // false

within_limit(None, 99)        // true
within_limit(Some(3), 2)      // true
within_limit(Some(3), 3)      // false

time_left(&ann, 40)           // Some(60)
time_left(&ann, 100)          // None

authorize(Some(&root), 99)    // Ok(())
authorize(Some(&ann), 99)     // Err(AuthError::NotAdmin)
authorize(Some(&root), 100)   // Err(AuthError::Expired)
authorize(None, 0)            // Err(AuthError::NoSession)

let mut slot = Some(root.clone());
evict_expired(&mut slot, 100) // Some(root): the session is taken out, and slot is now None
```
