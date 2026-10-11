`#[derive(PartialOrd, Ord)]` knows nothing about cards: it follows the order in which things are
**declared**.

- For an enum, a variant declared earlier is smaller than one declared later. Two values of the same
  variant compare their fields, so `Num(9) < Num(10)`.
- For a struct, the fields are compared one by one in declaration order, like a tuple: the first
  field decides, and the next one is looked at only on a tie.

The template lists suits and ranks from the strongest down, so every derived comparison was
reversed, and `Num(u8)`, declared last, even beat the `Ace`. `Card { suit, rank }` also compared
suits before ranks. Declaring the variants from the weakest to the strongest and putting `rank`
before `suit` fixes all of that without writing any comparison code. `PartialEq`, `Eq` and `Hash`
do not depend on the order, so `HashSet<Card>` worked either way.

`#[default]` marks a variant, not a position, so it has to move together with `Spades` and `Ace`.
If you reverse the lists but leave it on the first line, `Clubs` becomes the default suit, and
`#[default] Num(u8)` does not even compile: only a unit variant (one without fields) can be the
default. The derived `Default` of `Card` uses the default of each field, which gives the ace of
spades.

With the types right, the functions are short: `sort()`, `max()` and `HashSet` all use the derived
traits. `max()` returns an `Option<&Card>`, and `copied()` turns it into an `Option<Card>`.

### Alternatives

Keep the declarations as they were and implement the orders by hand. Then both `PartialOrd` and
`Ord` must leave the derive list, and `partial_cmp` simply returns `Some(self.cmp(other))`. Keeping
`#[derive(PartialOrd)]` next to a hand-written `Ord` compiles, but then `<` and `cmp` disagree, and
the standard library may use either of them: `sort()` and `max()`, for example, currently compare
with `<`, so they would not use your `Ord` at all.

```rust
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)] // no PartialOrd, Ord
pub enum Rank {
    #[default]
    Ace,
    King,
    Queen,
    Jack,
    Num(u8),
}

impl Rank {
    fn value(self) -> u8 {
        match self {
            Rank::Num(n) => n,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// `Card` also drops the derived PartialOrd and Ord.
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank).then(self.suit.cmp(&other.suit))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```

`Suit` gets its own `value` and the same two impls. Note that `Rank::value` relies on `n` being at
most 10: `Num(11)` would compare `Equal` to `Jack` without being `==` to it, which breaks the
contract of `Ord`. The derived order has no such corner case.

`distinct` can also sort the cards and remove the duplicates. `dedup` only removes **consecutive**
duplicates, so sort first.

```rust
pub fn distinct(cards: &[Card]) -> usize {
    let mut cards = sorted(cards);
    cards.dedup();
    cards.len()
}
```
