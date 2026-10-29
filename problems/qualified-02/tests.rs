use crate::judge::{case, check_eq};
use crate::user::{AgeWidget, Form, UsernameWidget, summary};

/// A widget pair of the harness's own.
struct Badge {
    nick: &'static str,
    level: u8,
}

impl UsernameWidget for Badge {
    fn get(&self) -> String {
        format!("@{}", self.nick)
    }
}

impl AgeWidget for Badge {
    fn get(&self) -> u8 {
        self.level
    }
}

fn form(username: &str, age: u8) -> Form {
    Form { username: username.to_string(), age }
}

pub fn run() {
    case("example-1", || check_eq(summary(&form("rustacean", 28)), "rustacean (28)"));
    case("case-1", || {
        // summary goes through the widget traits, so it works for other types too.
        check_eq(summary(&Badge { nick: "neo", level: 7 }), "@neo (7)")
    });
    case("case-2", || {
        check_eq(summary(&form("", 0)), " (0)")?;
        check_eq(summary(&form("max", 255)), "max (255)")
    });
    case("case-3", || {
        // Form still implements both widget traits as before.
        let f = form("ferris", 3);
        check_eq(<Form as UsernameWidget>::get(&f), "ferris")?;
        check_eq(<Form as AgeWidget>::get(&f), 3)
    });
}
