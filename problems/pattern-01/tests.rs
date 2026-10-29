use crate::judge::{case, check_eq};
use crate::user::filled_slots;

pub fn run() {
    case("example-1", || {
        check_eq(filled_slots(&[None, Some(4), Some(7)]), vec![(1, 4), (2, 7)])
    });
    case("case-1", || {
        // Empty input and only empty slots.
        check_eq(filled_slots(&[]), Vec::new())?;
        check_eq(filled_slots(&[None, None, None]), Vec::new())
    });
    case("case-2", || {
        // Positions count the empty slots, and Some(0) is a filled slot.
        check_eq(
            filled_slots(&[Some(0), None, None, Some(255), Some(3), None]),
            vec![(0, 0), (3, 255), (4, 3)],
        )
    });
}
