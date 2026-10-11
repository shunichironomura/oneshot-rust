`match self.state` with patterns that bind by value (`queue`, `session`) moves the payload out of
`self.state`. But `self` is only a `&mut Client`: the client belongs to the caller, and Rust never
lets a place behind a reference be left empty, not even until an arm puts a new value back (if
something in between panicked, the caller would be left holding a client whose state had been
moved away). That is E0507, and `take_closed` runs into it a second time with
`let closed = self.closed;`. The compiler's suggestions lead nowhere: `ref queue` or `&self.closed`
only borrow, but the next state has to **own** the queue (it becomes `sent` or `unsent`), and the
list of closed sessions has to own the `Session`, which may not implement `Clone`.

`std::mem::take(&mut self.state)` puts the default value in place and hands out the old state by
value, in a single step. `Conn` derives `Default` with `#[default]` on `Offline`, so the placeholder
is `Offline`. In `take_closed`, that is all there is to it: the empty `Vec` left behind is exactly
what the list should be afterwards.

In `step`, the placeholder is the trap. Writing `match std::mem::take(&mut self.state)` in the
template compiles, but the client is now `Offline` until an arm stores the next state, and the last
arm `_ => {}` stores nothing. "Nothing changes" has turned into "the state is lost": a successful
step disconnects an online client without closing its session, and a failed client comes back to
life as `Offline`. Once the old value has been taken out, **every** path has to put a state back,
including the ones that leave it unchanged.

The solution makes that impossible to forget. The whole `match` is the right-hand side of
`self.state = ...`, so every arm has to produce the next state (or the arms do not have the same
type), and `other => other` puts back the states that do not change.

`send` shows the other case: when a transition only changes the payload in place, matching on
`&mut self.state` is enough, and nothing has to be taken out.

### Alternatives

Keep the template's `match` statement and put the unchanged state back in the last arm.
`std::mem::replace` takes the placeholder as an argument, so it also works for types without a
`Default`. `drain(..)` empties a `Vec` and yields its items.

```rust
pub fn step(&mut self, ok: bool) {
    match std::mem::replace(&mut self.state, Conn::Offline) {
        Conn::Offline => {
            self.state = Conn::Connecting { attempts: 1, queue: Vec::new() };
        }
        Conn::Connecting { attempts, queue } => {
            self.state = if ok {
                Conn::Online(Session { attempts, sent: queue })
            } else if attempts < MAX_ATTEMPTS {
                Conn::Connecting { attempts: attempts + 1, queue }
            } else {
                Conn::Failed { unsent: queue }
            };
        }
        Conn::Online(session) if !ok => {
            self.closed.push(session);
            self.state = Conn::Offline;
        }
        other => self.state = other,
    }
}

pub fn take_closed(&mut self) -> Vec<Session> {
    self.closed.drain(..).collect()
}
```

You can also work in place and move out only what has to move. The attempt counter is a `u32`, so
`*attempts += 1` changes it through the reference, and `mem::take(queue)` takes just the queue. A
`Session` has no default value, so closing a session still takes the whole state. Here the states
that do not change are never taken out, so the empty last arm is correct.

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
