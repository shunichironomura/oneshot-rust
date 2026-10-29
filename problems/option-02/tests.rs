use crate::judge::{case, check_eq};
use crate::user::{Session, evict_expired, is_admin, time_left, within_limit};

fn session(user: &str, expires_at: u64, admin: bool) -> Session {
    Session { user: user.to_string(), expires_at, admin }
}

pub fn run() {
    case("example-1", || {
        check_eq(is_admin(Some(&session("root", 100, true))), true)?;
        check_eq(is_admin(None), false)?;
        check_eq(within_limit(None, 99), true)?;
        check_eq(within_limit(Some(3), 2), true)?;
        check_eq(within_limit(Some(3), 3), false)
    });
    case("example-2", || {
        let ann = session("ann", 100, false);
        check_eq(time_left(&ann, 40), Some(60))?;
        check_eq(time_left(&ann, 100), None)?;
        let mut slot = Some(session("root", 100, true));
        check_eq(evict_expired(&mut slot, 100), Some(session("root", 100, true)))?;
        check_eq(slot, None)
    });
    case("case-1", || {
        // Only the admin flag counts; the expiry time is not checked.
        check_eq(is_admin(Some(&session("ann", 100, false))), false)?;
        check_eq(is_admin(Some(&session("old", 0, true))), true)?;
        check_eq(is_admin(Some(&session("old", 0, false))), false)
    });
    case("case-2", || {
        // No limit allows every count, even u32::MAX; Some(n) allows exactly n attempts.
        check_eq(within_limit(None, 0), true)?;
        check_eq(within_limit(None, u32::MAX), true)?;
        check_eq(within_limit(Some(0), 0), false)?;
        check_eq(within_limit(Some(1), 0), true)?;
        check_eq(within_limit(Some(3), 4), false)?;
        check_eq(within_limit(Some(u32::MAX), u32::MAX - 1), true)?;
        check_eq(within_limit(Some(u32::MAX), u32::MAX), false)
    });
    case("case-3", || {
        // Well past the expiry time, and the extremes of u64.
        check_eq(time_left(&session("a", 100, true), 101), None)?;
        check_eq(time_left(&session("a", 5, false), u64::MAX), None)?;
        check_eq(time_left(&session("a", 0, false), 0), None)?;
        check_eq(time_left(&session("a", 1, false), 0), Some(1))?;
        check_eq(time_left(&session("a", u64::MAX, false), 0), Some(u64::MAX))?;
        check_eq(time_left(&session("a", u64::MAX, false), u64::MAX - 1), Some(1))
    });
    case("case-4", || {
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
    case("case-5", || {
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
