use crate::judge::{case, check_eq};
use crate::user::{Client, Conn, Session};

fn strings(xs: &[&str]) -> Vec<String> {
    xs.iter().map(|&s| String::from(s)).collect()
}

fn connecting(attempts: u32, queue: &[&str]) -> Conn {
    Conn::Connecting { attempts, queue: strings(queue) }
}

fn session(attempts: u32, sent: &[&str]) -> Session {
    Session { attempts, sent: strings(sent) }
}

fn online(attempts: u32, sent: &[&str]) -> Conn {
    Conn::Online(session(attempts, sent))
}

fn failed(unsent: &[&str]) -> Conn {
    Conn::Failed { unsent: strings(unsent) }
}

pub fn run() {
    case("sample-1", || {
        let mut c = Client::new();
        check_eq(c.send("lost"), false)?;
        c.step(true);
        check_eq(c.state(), &connecting(1, &[]))?;
        check_eq(c.send("hello"), true)?;
        c.step(false);
        check_eq(c.state(), &connecting(2, &["hello"]))?;
        c.step(true);
        check_eq(c.state(), &online(2, &["hello"]))?;
        check_eq(c.send("bye"), true)?;
        c.step(false);
        check_eq(c.state(), &Conn::Offline)?;
        check_eq(c.take_closed(), vec![session(2, &["hello", "bye"])])?;
        check_eq(c.take_closed(), vec![])
    });
    case("sample-2", || {
        let mut c = Client::new();
        c.step(true);
        check_eq(c.send("x"), true)?;
        c.step(false);
        c.step(false);
        check_eq(c.state(), &connecting(3, &["x"]))?;
        c.step(false);
        check_eq(c.state(), &failed(&["x"]))?;
        c.step(true);
        check_eq(c.state(), &failed(&["x"]))?;
        check_eq(c.send("y"), false)?;
        check_eq(c.state(), &failed(&["x"]))
    });
    case("hidden-1", || {
        // A new client is offline, and step leaves Offline whatever `ok` is.
        let mut c = Client::new();
        check_eq(c.state(), &Conn::Offline)?;
        check_eq(c.take_closed(), vec![])?;
        check_eq(c.send("nobody"), false)?;
        check_eq(c.state(), &Conn::Offline)?;
        c.step(false);
        check_eq(c.state(), &connecting(1, &[]))
    });
    case("hidden-2", || {
        // An online client stays online with the same session while steps succeed.
        let mut c = Client::new();
        c.step(true);
        c.send("a");
        c.step(true);
        check_eq(c.state(), &online(1, &["a"]))?;
        c.step(true);
        check_eq(c.send("b"), true)?;
        c.step(true);
        check_eq(c.state(), &online(1, &["a", "b"]))?;
        check_eq(c.take_closed(), vec![])
    });
    case("hidden-3", || {
        // Giving up with an empty queue; Failed is final and closes no session.
        let mut c = Client::new();
        for ok in [true, false, false, false] {
            c.step(ok);
        }
        check_eq(c.state(), &failed(&[]))?;
        c.step(false);
        c.step(true);
        check_eq(c.send("late"), false)?;
        check_eq(c.state(), &failed(&[]))?;
        check_eq(c.take_closed(), vec![])
    });
    case("hidden-4", || {
        // Several sessions: attempts and the queue start over each time, and closed
        // sessions are returned in order.
        let mut c = Client::new();
        c.step(true);
        c.send("a");
        c.step(true);
        c.step(false);
        check_eq(c.send("lost"), false)?;
        c.step(false);
        check_eq(c.state(), &connecting(1, &[]))?;
        c.step(false);
        c.send("b");
        c.step(true);
        c.send("c");
        c.step(false);
        check_eq(c.state(), &Conn::Offline)?;
        check_eq(c.take_closed(), vec![session(1, &["a"]), session(2, &["b", "c"])])?;
        check_eq(c.take_closed(), vec![])?;
        c.step(true);
        c.step(true);
        c.step(false);
        check_eq(c.take_closed(), vec![session(1, &[])])
    });
    case("hidden-5", || {
        // Queued messages keep their order across retries and become the session's
        // first messages. The third attempt may still succeed.
        let mut c = Client::new();
        c.step(true);
        c.send("x");
        c.send("y");
        c.step(false);
        c.send("z");
        c.step(false);
        check_eq(c.state(), &connecting(3, &["x", "y", "z"]))?;
        c.step(true);
        check_eq(c.send("w"), true)?;
        check_eq(c.state(), &online(3, &["x", "y", "z", "w"]))
    });
}
