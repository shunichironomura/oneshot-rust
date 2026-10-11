`match self.state` with patterns that bind by value (`queue`, `session`, `other`) moves the payload
out of `self.state`. But `self` is only a `&mut Client`: the client belongs to the caller, and Rust
never lets a place behind a reference be left empty, not even until `self.state = ...` puts a new
value back a moment later (if something in between panicked, the caller would be left holding a
client whose state had been moved away). That is E0507, and `take_closed` runs into it a second
time with `let closed = self.closed;`.

The compiler's suggestions lead nowhere here. `ref queue` or `&self.closed` only borrow, but the
next state has to **own** the queue (it becomes `sent` or `unsent`), and the list of closed sessions
has to own the `Session`. The note "consider implementing `Clone`" does not help either: the rules
forbid adding traits to the types, and a session is meant to be moved, not duplicated.

`std::mem::take(&mut self.state)` puts the default value in place and hands out the old state by
value, in a single step. `Conn` derives `Default` with `#[default]` on `Offline`, so the placeholder
is `Offline`. The `match` now owns the old state and can move its parts into the new one. Since the
whole `match` is still the right-hand side of `self.state = ...`, every arm must produce the next
state, and `other => other` puts back the states that do not change. In the same way,
`mem::take(&mut self.closed)` returns the list and leaves an empty `Vec` behind.

`send` shows the other case: when a transition only changes the payload in place, matching on
`&mut self.state` is enough.

A pitfall with `take`: if you turn the `match` into statements that assign `self.state` in some
arms only, the other arms leave the placeholder behind, and an online session or a failed client
silently becomes `Offline`.

### Alternatives

`std::mem::replace` takes the placeholder as an argument, so it also works for types without a
`Default`. `drain(..)` empties a `Vec` and yields its items.

```rust
pub fn step(&mut self, ok: bool) {
    self.state = match std::mem::replace(&mut self.state, Conn::Offline) {
        Conn::Offline => Conn::Connecting { attempts: 1, queue: Vec::new() },
        Conn::Connecting { attempts, queue } if ok => {
            Conn::Online(Session { attempts, sent: queue })
        }
        Conn::Connecting { attempts, queue } if attempts < MAX_ATTEMPTS => {
            Conn::Connecting { attempts: attempts + 1, queue }
        }
        Conn::Connecting { queue, .. } => Conn::Failed { unsent: queue },
        Conn::Online(session) if !ok => {
            self.closed.push(session);
            Conn::Offline
        }
        other => other,
    };
}

pub fn take_closed(&mut self) -> Vec<Session> {
    self.closed.drain(..).collect()
}
```

You can also work in place and move out only what has to move. The attempt counter is a `u32`, so
`*attempts += 1` changes it through the reference, and `mem::take(queue)` takes just the queue. A
`Session` has no default value, so closing a session still takes the whole state.

```rust
pub fn step(&mut self, ok: bool) {
    match &mut self.state {
        Conn::Offline => self.state = Conn::Connecting { attempts: 1, queue: Vec::new() },
        Conn::Connecting { attempts, queue } => {
            if ok {
                let sent = std::mem::take(queue);
                self.state = Conn::Online(Session { attempts: *attempts, sent });
            } else if *attempts < MAX_ATTEMPTS {
                *attempts += 1;
            } else {
                let unsent = std::mem::take(queue);
                self.state = Conn::Failed { unsent };
            }
        }
        Conn::Online(_) if !ok => {
            if let Conn::Online(session) = std::mem::take(&mut self.state) {
                self.closed.push(session);
            }
        }
        Conn::Online(_) | Conn::Failed { .. } => {}
    }
}
```
