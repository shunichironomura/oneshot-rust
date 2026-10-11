use crate::judge::{case, check_eq};
use crate::user::Profile;

fn profile(username: &str, nickname: Option<&str>, email: Option<&str>) -> Profile {
    Profile {
        username: username.to_string(),
        nickname: nickname.map(String::from),
        email: email.map(String::from),
    }
}

pub fn run() {
    case("sample-1", || check_eq(profile("alice", None, Some("a@b.c")).email_domain(), Some("b.c")));
    case("sample-2", || {
        let mut p = profile("alice", None, None);
        check_eq(p.set_nickname("ace"), None)?;
        check_eq(p.display_name(), "ace")?;
        check_eq(p.take_nickname().as_deref(), Some("ace"))?;
        check_eq(p.display_name(), "alice")
    });
    case("hidden-1", || {
        // The email is split at its first '@', and the domain may be empty.
        check_eq(profile("u", None, Some("a@b@c")).email_domain(), Some("b@c"))?;
        check_eq(profile("u", None, Some("abc@")).email_domain(), Some(""))?;
        check_eq(profile("u", None, Some("@example.com")).email_domain(), Some("example.com"))
    });
    case("hidden-2", || {
        // No email, or no '@' in it.
        check_eq(profile("u", Some("nick"), None).email_domain(), None)?;
        check_eq(profile("u", None, Some("abc")).email_domain(), None)?;
        check_eq(profile("u", None, Some("")).email_domain(), None)
    });
    case("hidden-3", || {
        // An empty nickname is still a nickname.
        check_eq(profile("bob", Some("Bobby"), None).display_name(), "Bobby")?;
        check_eq(profile("bob", Some(""), Some("b@c")).display_name(), "")?;
        check_eq(profile("bob", None, Some("b@c")).display_name(), "bob")
    });
    case("hidden-4", || {
        // Taking leaves no nickname behind, and a second take returns None.
        let mut p = profile("carol", Some("Caz"), Some("c@d.e"));
        check_eq(p.take_nickname().as_deref(), Some("Caz"))?;
        check_eq(&p, &profile("carol", None, Some("c@d.e")))?;
        check_eq(p.take_nickname(), None)?;
        check_eq(&p, &profile("carol", None, Some("c@d.e")))?;
        check_eq(p.display_name(), "carol")
    });
    case("hidden-5", || {
        // Each call returns the nickname it replaces.
        let mut p = profile("dave", Some("D"), Some("d@x"));
        check_eq(p.set_nickname("Davy").as_deref(), Some("D"))?;
        check_eq(p.set_nickname("ダイ").as_deref(), Some("Davy"))?;
        check_eq(&p, &profile("dave", Some("ダイ"), Some("d@x")))?;
        check_eq(p.display_name(), "ダイ")
    });
    case("hidden-6", || {
        // An empty nickname can be set and taken like any other.
        let mut p = profile("eve", None, None);
        check_eq(p.set_nickname(""), None)?;
        check_eq(p.display_name(), "")?;
        check_eq(p.take_nickname().as_deref(), Some(""))?;
        check_eq(p.take_nickname(), None)?;
        check_eq(p.display_name(), "eve")
    });
}
