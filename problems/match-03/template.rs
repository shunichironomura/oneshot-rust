pub fn digits_label(n: u8) -> &'static str {
    match n {
        0..10 => "one digit",
        10..100 => "two digits",
        100..255 => "three digits",
    }
}

pub fn sign_word(n: i32) -> &'static str {
    match n {
        x if x < 0 => "negative",
        0 => "zero",
        x if x > 0 => "positive",
    }
}

pub fn quadrant(x: i32, y: i32) -> &'static str {
    match (x.signum(), y.signum()) {
        (1, 1) => "I",
        (-1, 1) => "II",
        (-1, -1) => "III",
        (1, -1) => "IV",
        (0, 0) => "origin",
        (0, _) => "y-axis",
        (_, 0) => "x-axis",
    }
}

pub fn grade(score: u32) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
    }
}

pub fn parse_answer(s: &str) -> Option<bool> {
    match s {
        "yes" | "y" => Some(true),
        "no" | "n" => Some(false),
    }
}
