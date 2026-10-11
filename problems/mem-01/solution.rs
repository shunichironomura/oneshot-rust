use std::mem;

const MAX_ATTEMPTS: u32 = 3;

#[derive(Debug, PartialEq)]
pub struct Session {
    /// How many attempts it took to connect.
    pub attempts: u32,
    /// Messages sent during the session, oldest first.
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
    pub fn new() -> Client {
        Client::default()
    }

    pub fn state(&self) -> &Conn {
        &self.state
    }

    pub fn step(&mut self, ok: bool) {
        // Take the old state out, leaving the default `Offline` behind for a moment. The whole
        // `match` is the new state, so every arm has to put a state back, even an unchanged one.
        self.state = match mem::take(&mut self.state) {
            Conn::Offline => Conn::Connecting { attempts: 1, queue: Vec::new() },
            Conn::Connecting { attempts, queue } => {
                if ok {
                    Conn::Online(Session { attempts, sent: queue })
                } else if attempts < MAX_ATTEMPTS {
                    Conn::Connecting { attempts: attempts + 1, queue }
                } else {
                    Conn::Failed { unsent: queue }
                }
            }
            Conn::Online(session) if !ok => {
                self.closed.push(session);
                Conn::Offline
            }
            // Online with `ok`, and Failed: put the state back unchanged.
            other => other,
        };
    }

    pub fn send(&mut self, msg: &str) -> bool {
        match &mut self.state {
            Conn::Connecting { queue, .. } => queue.push(msg.to_string()),
            Conn::Online(session) => session.sent.push(msg.to_string()),
            Conn::Offline | Conn::Failed { .. } => return false,
        }
        true
    }

    pub fn take_closed(&mut self) -> Vec<Session> {
        mem::take(&mut self.closed)
    }
}
