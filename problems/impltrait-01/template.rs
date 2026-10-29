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
