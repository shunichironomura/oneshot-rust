The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

A `Client` keeps a network connection up. Its state is a `Conn`, and messages sent while it is still
connecting wait in a queue until the connection is up. A `Session` is one connection that came up:
`attempts` is how many attempts it took to connect, and `sent` holds the messages of the session,
oldest first. The types are already in the editor.

```rust
const MAX_ATTEMPTS: u32 = 3;

#[derive(Debug, PartialEq)]
pub struct Session {
    pub attempts: u32,
    pub sent: Vec<String>,
}

#[derive(Debug, Default, PartialEq)]
pub enum Conn {
    #[default]
    Offline,
    Connecting { attempts: u32, queue: Vec<String> },
    Online(Session),
    Failed { unsent: Vec<String> },
}

#[derive(Default)]
pub struct Client {
    state: Conn,
    closed: Vec<Session>,
}

impl Client {
    pub fn new() -> Client;
    pub fn state(&self) -> &Conn;
    pub fn step(&mut self, ok: bool);
    pub fn send(&mut self, msg: &str) -> bool;
    pub fn take_closed(&mut self) -> Vec<Session>;
}
```

- `new` creates a client that is `Offline` and has no closed sessions. `state` returns the current
  state.
- `step(ok)` is one tick of time, and `ok` tells whether the network answered during it. The state
  changes as follows:
  - `Offline` becomes `Connecting { attempts: 1, queue: [] }`, whatever `ok` is.
  - `Connecting { attempts, queue }`:
    - if `ok`, it becomes `Online(Session { attempts, sent: queue })`: the queued messages become
      the first messages of the session.
    - otherwise, if `attempts < 3` (`MAX_ATTEMPTS`), it becomes
      `Connecting { attempts: attempts + 1, queue }`, keeping the queue.
    - otherwise (the third attempt failed too), it becomes `Failed { unsent: queue }`.
  - `Online(session)` stays unchanged if `ok`. Otherwise the connection is lost: the state becomes
    `Offline`, and `session` is added to the end of the client's closed sessions.
  - `Failed { .. }` is final: `step` never changes it.
- `send(msg)` appends `msg` to `queue` while `Connecting` and to the session's `sent` while
  `Online`, and returns `true`. While `Offline` or `Failed`, it changes nothing and returns `false`.
- `take_closed` returns the closed sessions in the order they were closed and empties the list, so
  each session is returned only once.

Do not change the type definitions (including their derives) or the public signatures.

### Examples

```rust
let mut c = Client::new();
c.send("lost");  // false: offline
c.step(true);    // Connecting { attempts: 1, queue: [] }
c.send("hello"); // true
c.step(false);   // Connecting { attempts: 2, queue: ["hello"] }
c.step(true);    // Online(Session { attempts: 2, sent: ["hello"] })
c.send("bye");   // true
c.step(false);   // Offline
c.take_closed(); // [Session { attempts: 2, sent: ["hello", "bye"] }]
c.take_closed(); // []
```

From a new client, `step(true)`, `send("x")` and then three `step(false)` lead to
`Failed { unsent: ["x"] }`, which no later `step` changes. `send("y")` then returns `false`.
