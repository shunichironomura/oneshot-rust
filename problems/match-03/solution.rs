use std::cmp::Ordering::{Equal, Greater, Less};

pub fn quadrant(x: i32, y: i32) -> &'static str {
    match (x.cmp(&0), y.cmp(&0)) {
        (Greater, Greater) => "I",
        (Less, Greater) => "II",
        (Less, Less) => "III",
        (Greater, Less) => "IV",
        (Equal, Equal) => "origin",
        (Equal, _) => "y-axis",
        (_, Equal) => "x-axis",
    }
}
