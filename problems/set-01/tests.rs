use crate::judge::case;
use crate::user::{Overlap, overlap};

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
    case("example-1", || {
        check_overlap(
            overlap(&["b", "a", "c", "a"], &["d", "c", "b"]),
            [&["b", "c"], &["a"], &["d"], &["a", "d"], &["a", "b", "c", "d"]],
        )
    });
    case("case-1", || {
        // Empty lists, and a word repeated within a list counts once.
        check_overlap(overlap(&[], &[]), [&[], &[], &[], &[], &[]])?;
        let ab: &[&str] = &["a", "b"];
        check_overlap(overlap(&["b", "a", "a"], &[]), [&[], ab, &[], ab, ab])
    });
    case("case-2", || {
        // The same words with different counts and orders.
        let xy: &[&str] = &["x", "y"];
        check_overlap(overlap(&["y", "x"], &["x", "y", "x"]), [xy, &[], &[], &[], xy])
    });
    case("case-3", || {
        // Words are case-sensitive and sorted by bytes. "Z" is repeated only in `right`.
        check_overlap(
            overlap(&["b", "é", "a", "apple"], &["Z", "a", "app", "B", "Z"]),
            [
                &["a"],
                &["apple", "b", "é"],
                &["B", "Z", "app"],
                &["B", "Z", "app", "apple", "b", "é"],
                &["B", "Z", "a", "app", "apple", "b", "é"],
            ],
        )
    });
}
