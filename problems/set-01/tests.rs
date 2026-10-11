use crate::judge::{case, check_eq};
use crate::user::{Overlap, first_repeat, overlap};

/// Compares the fields with `[both, only_left, only_right, exactly_one, all]`.
fn check_overlap(got: Overlap, want: [&[&str]; 5]) -> Result<(), String> {
    let fields = [
        ("both", &got.both),
        ("only_left", &got.only_left),
        ("only_right", &got.only_right),
        ("exactly_one", &got.exactly_one),
        ("all", &got.all),
    ];
    for ((name, got), want) in fields.into_iter().zip(want) {
        if got != want {
            return Err(format!("{name}: expected {want:?}, got {got:?}"));
        }
    }
    Ok(())
}

pub fn run() {
    case("sample-1", || {
        check_overlap(
            overlap(&["b", "a", "c", "a"], &["d", "c", "b"]),
            [&["b", "c"], &["a"], &["d"], &["a", "d"], &["a", "b", "c", "d"]],
        )
    });
    case("sample-2", || check_eq(first_repeat(&["a", "b", "c", "b", "a"]), Some("b")));
    case("hidden-1", || check_overlap(overlap(&[], &[]), [&[], &[], &[], &[], &[]]));
    case("hidden-2", || {
        // One list is empty, and a word repeated within a list counts once.
        let ab: &[&str] = &["a", "b"];
        check_overlap(overlap(&["b", "a", "a"], &[]), [&[], ab, &[], ab, ab])
    });
    case("hidden-3", || {
        // The same words with different counts and orders.
        let xy: &[&str] = &["x", "y"];
        check_overlap(overlap(&["y", "x"], &["x", "y", "x"]), [xy, &[], &[], &[], xy])
    });
    case("hidden-4", || {
        // Words are case-sensitive and sorted by bytes.
        check_overlap(
            overlap(&["b", "é", "a", "apple"], &["a", "Z", "app", "B", "z"]),
            [
                &["a"],
                &["apple", "b", "é"],
                &["B", "Z", "app", "z"],
                &["B", "Z", "app", "apple", "b", "z", "é"],
                &["B", "Z", "a", "app", "apple", "b", "z", "é"],
            ],
        )
    });
    case("hidden-5", || {
        check_eq(first_repeat(&["x", "x"]), Some("x"))?;
        check_eq(first_repeat(&["a", "b", "b", "a"]), Some("b"))?;
        check_eq(first_repeat(&["A", "b", "a", "b"]), Some("b"))
    });
    case("hidden-6", || {
        check_eq(first_repeat(&[]), None)?;
        check_eq(first_repeat(&["a", "A", "b"]), None)
    });
    case("hidden-7", || {
        // The result borrows from the text, so it outlives the vector of words.
        let text = String::from("up down up");
        let found = {
            let words: Vec<&str> = text.split(' ').collect();
            first_repeat(&words)
        };
        check_eq(found, Some("up"))
    });
}
