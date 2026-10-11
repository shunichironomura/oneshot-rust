use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keep {
    Evens,
    Odds,
    All,
}

pub fn select(v: &[i32], keep: Keep) -> impl Iterator<Item = i32> + '_ {
    match keep {
        Keep::Evens => v.iter().copied().filter(|x| x % 2 == 0),
        Keep::Odds => v.iter().copied().filter(|x| x % 2 == 1),
        Keep::All => v.iter().copied(),
    }
}

pub fn describe(n: i32) -> impl Display {
    if n < 0 {
        format!("minus {}", -n)
    } else {
        n
    }
}

pub fn op_for(symbol: char, scale: i32) -> impl Fn(i32, i32) -> i32 {
    match symbol {
        '+' => move |a, b| (a + b) * scale,
        '-' => move |a, b| (a - b) * scale,
        '*' => move |a, b| a * b * scale,
        _ => move |a, _| a * scale,
    }
}
