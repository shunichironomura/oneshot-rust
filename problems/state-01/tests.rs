use crate::judge::{case, check_eq};
use crate::user::{Closed, Connection, Open};

pub fn run() {
    case("sample-1", || {
        let mut conn = Connection::new("db").open();
        conn.send("hi");
        let conn = conn.close();
        let expected = ["open", "send hi", "close"].map(String::from);
        check_eq((conn.host(), conn.log()), ("db", &expected[..]))
    });
    case("hidden-1", || {
        let conn = Connection::new("example.com");
        check_eq((conn.host(), conn.log().len()), ("example.com", 0))
    });
    case("hidden-2", || {
        let mut conn: Connection<Open> = Connection::new("a").open();
        conn.send("");
        conn.send("x y");
        let expected = ["open", "send ", "send x y"].map(String::from);
        check_eq((conn.host(), conn.log()), ("a", &expected[..]))
    });
    case("hidden-3", || {
        let mut conn: Connection<Open> = Connection::new("h").open();
        conn.send("1");
        let closed: Connection<Closed> = conn.close();
        let mut conn = closed.open();
        conn.send("2");
        let conn = conn.close().open().close();
        let expected = ["open", "send 1", "close", "open", "send 2", "close", "open", "close"]
            .map(String::from);
        check_eq(conn.log(), &expected[..])
    });
}
