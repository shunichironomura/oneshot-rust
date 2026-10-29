use crate::judge::{case, check_eq};
use crate::user::{AgeWidget, Form, UsernameWidget, kinds};

/// A widget pair of the harness's own, with a `kind` of its own.
struct Badge;

impl UsernameWidget for Badge {
    fn kind() -> &'static str {
        "handle"
    }
}

impl AgeWidget for Badge {}

pub fn run() {
    case("example-1", || check_eq(kinds::<Form>(), ("username", "age")));
    case("case-1", || {
        // kinds asks W itself, so an overridden kind shows up.
        check_eq(kinds::<Badge>(), ("handle", "age"))
    });
    case("case-2", || {
        // Form still implements both traits as before.
        check_eq(<Form as UsernameWidget>::kind(), "username")?;
        check_eq(<Form as AgeWidget>::kind(), "age")
    });
}
