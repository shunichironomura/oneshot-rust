use crate::judge::{case, check_eq};
use crate::user::{Session, evict_expired};

fn session(user: &str, expires_at: u64, admin: bool) -> Session {
    Session { user: user.to_string(), expires_at, admin }
}

pub fn run() {
    case("example-1", || {
        let mut slot = Some(session("root", 100, true));
        check_eq(evict_expired(&mut slot, 100), Some(session("root", 100, true)))?;
        check_eq(slot, None)
    });
    case("case-1", || {
        // A valid session stays in its slot, and an empty slot stays empty.
        let mut slot = Some(session("bob", 50, false));
        check_eq(evict_expired(&mut slot, 49), None)?;
        check_eq(&slot, &Some(session("bob", 50, false)))?;
        check_eq(evict_expired(&mut slot, 0), None)?;
        check_eq(&slot, &Some(session("bob", 50, false)))?;
        let mut empty: Option<Session> = None;
        check_eq(evict_expired(&mut empty, 7), None)?;
        check_eq(empty, None)
    });
    case("case-2", || {
        // An expired session is taken out once; after that the slot is empty.
        let mut slot = Some(session("ゆき", 30, true));
        check_eq(evict_expired(&mut slot, 31), Some(session("ゆき", 30, true)))?;
        check_eq(&slot, &None)?;
        check_eq(evict_expired(&mut slot, 31), None)?;
        let mut slot = Some(session("z", 0, false));
        check_eq(evict_expired(&mut slot, 0), Some(session("z", 0, false)))?;
        check_eq(slot, None)
    });
}
