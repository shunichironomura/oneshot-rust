use std::fmt;

use crate::judge::{case, check_eq};
use crate::user::{Outline, Point, Tags};

/// A type of the harness: `Outline` must work for it through the default method alone.
struct Label(&'static str);

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl Outline for Label {}

fn tags(xs: &[&str]) -> Tags {
    Tags(xs.iter().map(|s| s.to_string()).collect())
}

pub fn run() {
    case("sample-1", || {
        let p = Point { x: 1, y: 3 };
        check_eq(p.to_string(), "(1, 3)")?;
        check_eq(p.outline(), "**********\n*        *\n* (1, 3) *\n*        *\n**********")
    });
    case("sample-2", || {
        check_eq(tags(&["red", "green", "blue"]).to_string(), "[red, green, blue]")?;
        check_eq(tags(&["solo"]).to_string(), "[solo]")
    });
    case("hidden-1", || {
        // No strings: "[]", and the frame is 2 + 4 = 6 wide.
        let t = tags(&[]);
        check_eq(t.to_string(), "[]")?;
        check_eq(t.outline(), "******\n*    *\n* [] *\n*    *\n******")
    });
    case("hidden-2", || {
        check_eq(Point { x: -12, y: 0 }.to_string(), "(-12, 0)")?;
        check_eq(
            Point { x: i32::MIN, y: i32::MAX }.to_string(),
            "(-2147483648, 2147483647)",
        )?;
        check_eq(
            Point { x: -7, y: -250 }.outline(),
            "**************\n*            *\n* (-7, -250) *\n*            *\n**************",
        )
    });
    case("hidden-3", || {
        // A type of the harness, with a non-ASCII character: 5 chars, but 6 bytes.
        check_eq(
            Label("héllo").outline(),
            "*********\n*       *\n* héllo *\n*       *\n*********",
        )
    });
    case("hidden-4", || {
        // An empty text gives a frame 4 wide.
        check_eq(Label("").outline(), "****\n*  *\n*  *\n*  *\n****")
    });
    case("hidden-5", || {
        // 16 chars, 18 bytes.
        let t = tags(&["Zürich", "Genève"]);
        check_eq(t.to_string(), "[Zürich, Genève]")?;
        check_eq(
            t.outline(),
            "********************\n*                  *\n* [Zürich, Genève] *\n*                  *\n********************",
        )
    });
    case("hidden-6", || {
        // Empty strings are still separated by ", ".
        check_eq(tags(&["", "x", ""]).to_string(), "[, x, ]")?;
        check_eq(tags(&[""]).to_string(), "[]")
    });
}
