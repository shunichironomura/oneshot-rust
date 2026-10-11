use crate::judge::{case, check_eq};
use crate::user::{AgeWidget, Form, Labeled, UsernameWidget, kinds, next_age, summary, tag};

/// A widget pair of the harness's own, with a `kind` of its own.
struct Badge {
    nick: &'static str,
    level: u8,
}

impl UsernameWidget for Badge {
    fn get(&self) -> String {
        format!("@{}", self.nick)
    }

    fn kind() -> &'static str {
        "handle"
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
    case("sample-1", || check_eq(summary(&form("rustacean", 28)), "rustacean (28)"));
    case("sample-2", || check_eq(kinds::<Form>(), ("username", "age")));
    case("sample-3", || {
        let f = form("rustacean", 28);
        check_eq(next_age(&f), 29)?;
        check_eq(tag(&f), "user:rustacean")
    });
    case("hidden-1", || {
        // The age stays at 255 instead of overflowing.
        check_eq(next_age(&form("old", 255)), 255)?;
        check_eq(next_age(&form("almost", 254)), 255)?;
        check_eq(next_age(&form("baby", 0)), 1)
    });
    case("hidden-2", || {
        // tag uses the Labeled impl; the inherent label is still there and unchanged.
        let f = form("ferris", 3);
        check_eq(tag(&f), "user:ferris")?;
        check_eq(f.label(), "form of ferris")?;
        check_eq(Labeled::label(&f), "user:ferris")?;
        check_eq(tag(&form("", 1)), "user:")
    });
    case("hidden-3", || {
        // summary goes through the widget traits, so it works for other types too.
        check_eq(summary(&Badge { nick: "neo", level: 7 }), "@neo (7)")?;
        check_eq(summary(&form("", 0)), " (0)")?;
        check_eq(summary(&form("max", 255)), "max (255)")
    });
    case("hidden-4", || {
        // kinds asks W itself, so an overridden kind shows up.
        check_eq(kinds::<Badge>(), ("handle", "age"))
    });
    case("hidden-5", || {
        // Form still implements both widget traits as before.
        let f = form("ferris", 3);
        check_eq(<Form as UsernameWidget>::get(&f), "ferris")?;
        check_eq(<Form as AgeWidget>::get(&f), 3)?;
        check_eq(<Form as UsernameWidget>::kind(), "username")?;
        check_eq(<Badge as AgeWidget>::kind(), "age")
    });
}
