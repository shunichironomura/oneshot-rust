use crate::judge::{case, check_eq};
use crate::user::{filled_slots, first_even, title_or_default};

fn titled(given: bool, title: &str) -> (bool, String) {
    (given, title.to_string())
}

fn some(title: &str) -> Option<String> {
    Some(title.to_string())
}

pub fn run() {
    case("sample-1", || {
        check_eq(first_even(&[1, 4, 6]), Some(4))?;
        check_eq(title_or_default(some("Notes")), titled(true, "Notes"))?;
        check_eq(title_or_default(None), titled(false, "Untitled"))
    });
    case("sample-2", || {
        check_eq(filled_slots(&[None, Some(4), Some(7)]), vec![(1, 4), (2, 7)])
    });
    case("hidden-1", || {
        // No even element at all, including negative odd numbers.
        check_eq(first_even(&[]), None)?;
        check_eq(first_even(&[1, 3, -5, 7]), None)
    });
    case("hidden-2", || {
        // Zero and negative numbers can be even; -3 is odd.
        check_eq(first_even(&[-2]), Some(-2))?;
        check_eq(first_even(&[7, 0, 2]), Some(0))?;
        check_eq(first_even(&[-3, 9, -8, 6]), Some(-8))
    });
    case("hidden-3", || {
        // The given title is returned unchanged, even if it is empty or "Untitled".
        check_eq(title_or_default(some("")), titled(true, ""))?;
        check_eq(title_or_default(some("Untitled")), titled(true, "Untitled"))?;
        check_eq(title_or_default(some(" Draft 2 ")), titled(true, " Draft 2 "))
    });
    case("hidden-4", || {
        // Empty input and only empty slots.
        check_eq(filled_slots(&[]), Vec::new())?;
        check_eq(filled_slots(&[None, None, None]), Vec::new())
    });
    case("hidden-5", || {
        // Positions count the empty slots, and Some(0) is a filled slot.
        check_eq(
            filled_slots(&[Some(0), None, None, Some(255), Some(3), None]),
            vec![(0, 0), (3, 255), (4, 3)],
        )
    });
}
