use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};

use crate::judge::{case, check, check_eq};
use crate::user::Rank::{Ace, Jack, King, Num, Queen};
use crate::user::Suit::{Clubs, Diamonds, Hearts, Spades};
use crate::user::{Card, Rank, Suit, best, distinct, sorted};

fn card(rank: Rank, suit: Suit) -> Card {
    Card { rank, suit }
}

pub fn run() {
    case("sample-1", || {
        let cards = [card(King, Hearts), card(Num(10), Spades), card(King, Clubs)];
        check_eq(
            sorted(&cards),
            vec![card(Num(10), Spades), card(King, Clubs), card(King, Hearts)],
        )?;
        check_eq(best(&cards), Some(card(King, Hearts)))
    });
    case("sample-2", || {
        check_eq(distinct(&[card(Ace, Spades), card(Ace, Spades), card(Num(2), Clubs)]), 2)?;
        check(Num(10) < Jack, "Rank::Num(10) < Rank::Jack should be true")?;
        check_eq(Card::default(), card(Ace, Spades))
    });
    case("hidden-1", || {
        // Ranks, with `<`, `cmp` and `sort()`.
        let chain = [Num(2), Num(3), Num(9), Num(10), Jack, Queen, King, Ace];
        for w in chain.windows(2) {
            check(w[0] < w[1], "ranks are out of order")?;
            check_eq(w[1].cmp(&w[0]), Ordering::Greater)?;
        }
        let mut ranks = vec![Ace, Num(10), Queen, Num(2), King, Jack, Num(9), Num(3)];
        ranks.sort();
        check_eq(ranks, chain.to_vec())?;
        check_eq(Num(7).cmp(&Num(7)), Ordering::Equal)
    });
    case("hidden-2", || {
        // Suits, and the default values of each type.
        check(Clubs < Diamonds && Diamonds < Hearts && Hearts < Spades, "suits are out of order")?;
        let mut suits = vec![Hearts, Spades, Clubs, Diamonds];
        suits.sort();
        check_eq(suits, vec![Clubs, Diamonds, Hearts, Spades])?;
        check_eq(Suit::default(), Spades)?;
        check_eq(Rank::default(), Ace)?;
        check_eq(Card::default(), Card { rank: Rank::default(), suit: Suit::default() })
    });
    case("hidden-3", || {
        // The rank decides first, even against a stronger suit; the suit breaks ties.
        check(card(Num(10), Spades) < card(Jack, Clubs), "10S < JC should be true")?;
        check(card(Ace, Clubs) > card(King, Spades), "AC > KS should be true")?;
        check(card(Num(2), Spades) < card(Num(3), Clubs), "2S < 3C should be true")?;
        check(card(Queen, Diamonds) < card(Queen, Hearts), "QD < QH should be true")?;
        check_eq(card(Num(7), Hearts).partial_cmp(&card(Num(7), Clubs)), Some(Ordering::Greater))?;
        check_eq(card(Jack, Spades).cmp(&card(Queen, Clubs)), Ordering::Less)?;
        check_eq(card(Jack, Diamonds).cmp(&card(Jack, Diamonds)), Ordering::Equal)
    });
    case("hidden-4", || {
        // A longer list: `sorted`, `sort()` and a `BTreeSet` must agree.
        let cards = vec![
            card(Queen, Spades),
            card(Num(2), Hearts),
            card(Ace, Clubs),
            card(Num(10), Diamonds),
            card(Num(2), Clubs),
            card(Queen, Spades),
            card(Num(9), Spades),
            card(Jack, Hearts),
            card(Ace, Diamonds),
        ];
        let expected = vec![
            card(Num(2), Clubs),
            card(Num(2), Hearts),
            card(Num(9), Spades),
            card(Num(10), Diamonds),
            card(Jack, Hearts),
            card(Queen, Spades),
            card(Queen, Spades),
            card(Ace, Clubs),
            card(Ace, Diamonds),
        ];
        check_eq(sorted(&cards), expected.clone())?;
        check_eq(sorted(&[]), Vec::new())?;
        let mut by_sort = cards.clone();
        by_sort.sort();
        check_eq(&by_sort, &expected)?;
        let mut set = BTreeSet::new();
        for &c in &cards {
            set.insert(c);
        }
        let mut unique = expected;
        unique.dedup();
        check_eq(set.into_iter().collect::<Vec<_>>(), unique)
    });
    case("hidden-5", || {
        check_eq(best(&[]), None)?;
        check_eq(best(&[card(Num(3), Diamonds)]), Some(card(Num(3), Diamonds)))?;
        check_eq(best(&[card(Num(4), Hearts); 3]), Some(card(Num(4), Hearts)))?;
        check_eq(best(&[card(Num(10), Hearts), card(Num(9), Spades)]), Some(card(Num(10), Hearts)))?;
        let cards = [
            card(Ace, Clubs),
            card(King, Spades),
            card(Ace, Diamonds),
            card(Num(10), Spades),
            card(Ace, Clubs),
        ];
        check_eq(best(&cards), Some(card(Ace, Diamonds)))?;
        check_eq(cards.iter().max(), Some(&card(Ace, Diamonds)))
    });
    case("hidden-6", || {
        check_eq(distinct(&[]), 0)?;
        check_eq(distinct(&[card(Num(2), Clubs), card(Ace, Clubs), card(Num(2), Clubs)]), 2)?;
        check_eq(
            distinct(&[card(King, Clubs), card(King, Diamonds), card(King, Hearts), card(King, Spades)]),
            4,
        )?;
        let set: HashSet<Card> = [card(Jack, Spades), card(Jack, Spades), Card::default()].into_iter().collect();
        check_eq(set.len(), 2)?;
        check(set.contains(&card(Ace, Spades)), "the set should contain the ace of spades")
    });
}
