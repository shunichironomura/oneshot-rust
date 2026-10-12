use std::cmp::Ordering::{Equal, Greater, Less};

pub fn digits_label(n: u8) -> &'static str {
    match n {
        0..10 => "one digit",
        10..100 => "two digits",
        100.. => "three digits",
    }
}

pub fn sign_word(n: i32) -> &'static str {
    match n {
        ..0 => "negative",
        0 => "zero",
        1.. => "positive",
    }
}

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
