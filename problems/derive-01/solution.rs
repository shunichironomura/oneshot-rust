use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    #[default]
    Spades,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
    /// A number card from 2 to 10.
    Num(u8),
    Jack,
    Queen,
    King,
    #[default]
    Ace,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

pub fn sorted(cards: &[Card]) -> Vec<Card> {
    let mut cards = cards.to_vec();
    cards.sort();
    cards
}

pub fn best(cards: &[Card]) -> Option<Card> {
    cards.iter().max().copied()
}

pub fn distinct(cards: &[Card]) -> usize {
    cards.iter().collect::<HashSet<_>>().len()
}
