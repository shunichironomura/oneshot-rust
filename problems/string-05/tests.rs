use crate::judge::{case, check_eq};
use crate::user::initials;

pub fn run() {
    case("example-1", || check_eq(initials("Grace Brewster Hopper"), "G.B.H."));
    case("example-2", || check_eq(initials("émile zola"), "é.z."));
    case("case-1", || {
        // No words at all, and a single word.
        check_eq(initials(""), "")?;
        check_eq(initials(" \t "), "")?;
        check_eq(initials("Plato"), "P.")
    });
    case("case-2", || {
        // Any run of whitespace separates words, and the case is kept.
        check_eq(initials("  Guido \t van\nRossum  "), "G.v.R.")
    });
    case("case-3", || {
        // First characters of 2, 3 and 4 bytes in UTF-8.
        check_eq(initials("Ørjan 山田 🦀crab"), "Ø.山.🦀.")
    });
}
