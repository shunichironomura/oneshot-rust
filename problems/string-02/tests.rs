use crate::judge::{case, check_eq};
use crate::user::{initials, join_path, shout};

fn s(text: &str) -> String {
    text.to_string()
}

pub fn run() {
    case("sample-1", || {
        check_eq(initials("Grace Brewster Hopper"), "G.B.H.")?;
        check_eq(initials("émile zola"), "é.z.")
    });
    case("sample-2", || check_eq(join_path(s("usr"), s("bin")), "usr/bin"));
    case("sample-3", || check_eq(shout(s("hey")), "HEY!"));
    case("hidden-1", || {
        // No words at all, and a single word.
        check_eq(initials(""), "")?;
        check_eq(initials(" \t "), "")?;
        check_eq(initials("Plato"), "P.")
    });
    case("hidden-2", || {
        // Any run of whitespace separates words, and the case is kept.
        check_eq(initials("  Guido \t van\nRossum  "), "G.v.R.")
    });
    case("hidden-3", || {
        // First characters of 2, 3 and 4 bytes in UTF-8.
        check_eq(initials("Ørjan 山田 🦀crab"), "Ø.山.🦀.")
    });
    case("hidden-4", || {
        // An empty dir returns file unchanged; an empty file still gets the slash.
        check_eq(join_path(s(""), s("a.txt")), "a.txt")?;
        check_eq(join_path(s(""), s("")), "")?;
        check_eq(join_path(s("usr"), s("")), "usr/")?;
        check_eq(join_path(s("/home/ferris"), s("src/main.rs")), "/home/ferris/src/main.rs")
    });
    case("hidden-5", || {
        // An empty message stays empty, without the "!".
        check_eq(shout(s("")), "")?;
        check_eq(shout(s("!")), "!!")
    });
    case("hidden-6", || {
        // Unicode uppercasing: some characters become several.
        check_eq(shout(s("straße")), "STRASSE!")?;
        check_eq(shout(s("Ok, élan 2")), "OK, ÉLAN 2!")
    });
}
