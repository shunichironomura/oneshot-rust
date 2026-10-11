use std::marker::PhantomData;

pub struct Closed;
pub struct Open;

pub struct Connection<State> {
    host: String,
    log: Vec<String>,
    _state: PhantomData<State>,
}

impl Connection<Closed> {
    pub fn new(host: &str) -> Connection<Closed> {
        Connection {
            host: host.to_string(),
            log: Vec::new(),
            _state: PhantomData,
        }
    }

    pub fn open(self) -> Connection<Open> {
        self.transition("open")
    }
}

impl Connection<Open> {
    pub fn send(&mut self, message: &str) {
        self.log.push(format!("send {message}"));
    }

    pub fn close(self) -> Connection<Closed> {
        self.transition("close")
    }
}

impl<State> Connection<State> {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn log(&self) -> &[String] {
        &self.log
    }

    fn transition<Next>(mut self, event: &str) -> Connection<Next> {
        self.log.push(event.to_string());
        Connection {
            host: self.host,
            log: self.log,
            _state: PhantomData,
        }
    }
}
