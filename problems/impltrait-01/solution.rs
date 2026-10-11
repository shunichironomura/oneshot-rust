use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keep {
    Evens,
    Odds,
    All,
}

pub fn select(v: &[i32], keep: Keep) -> impl Iterator<Item = i32> + '_ {
    v.iter().copied().filter(move |&x| match keep {
        Keep::Evens => x % 2 == 0,
        Keep::Odds => x % 2 != 0,
        Keep::All => true,
    })
}

pub fn describe(n: i32) -> impl Display {
    if n < 0 {
        format!("minus {}", n.unsigned_abs())
    } else {
        n.to_string()
    }
}

pub fn op_for(symbol: char, scale: i32) -> impl Fn(i32, i32) -> i32 {
    move |a, b| match symbol {
        '+' => (a + b) * scale,
        '-' => (a - b) * scale,
        '*' => a * b * scale,
        _ => a * scale,
    }
}
