#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Suit {
    #[default]
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
    #[default]
    Ace,
    King,
    Queen,
    Jack,
    /// A number card from 2 to 10.
    Num(u8),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn sorted(cards: &[Card]) -> Vec<Card> {
    todo!()
}

pub fn best(cards: &[Card]) -> Option<Card> {
    todo!()
}

pub fn distinct(cards: &[Card]) -> usize {
    todo!()
}
