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
