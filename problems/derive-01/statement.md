The editor already contains three types for playing cards: `Suit`, `Rank` and `Card`. Each of them
derives `Clone`, `Copy`, `Debug`, `Default`, `PartialEq`, `Eq`, `PartialOrd`, `Ord` and `Hash`.
Make everything follow the rules below, and implement three functions.

```rust
pub fn sorted(cards: &[Card]) -> Vec<Card>
pub fn best(cards: &[Card]) -> Option<Card>
pub fn distinct(cards: &[Card]) -> usize
```

- `Suit` has four variants, ordered `Clubs` < `Diamonds` < `Hearts` < `Spades`.
- `Rank` has the variants `Num(u8)`, `Jack`, `Queen`, `King` and `Ace`. `Num(n)` is the number
  card `n`, where `n` is from 2 to 10 (the tests use no other numbers). Number cards are ordered by
  `n`, every number card is lower than `Jack`, and `Jack` < `Queen` < `King` < `Ace`.
- `Card` has two public fields, `rank: Rank` and `suit: Suit`. Cards are compared by `rank` first,
  and cards of the same rank by `suit`. Two cards are equal when both fields are equal.
- `Suit::default()` is `Spades`, `Rank::default()` is `Ace`, and `Card::default()` is the ace of
  spades.
- These orders and default values must hold for the types themselves: besides calling the three
  functions, the tests compare values with `<`, `>`, `partial_cmp` and `cmp`, call `sort()` and
  `max()`, and put cards in a `BTreeSet` and a `HashSet`.
- `sorted` returns all the cards (duplicates included) in ascending order. `best` returns the
  greatest card, or `None` if `cards` is empty. `distinct` returns the number of different cards.
- You may change the type definitions (for example, edit the derives or implement traits by hand),
  as long as the names of the types, variants and fields stay the same, the fields stay `pub`, and
  all nine traits are still implemented.

### Examples

Below, `K♥` stands for `Card { rank: Rank::King, suit: Suit::Hearts }`, `10♠` for
`Card { rank: Rank::Num(10), suit: Suit::Spades }`, and so on.

- `sorted(&[K♥, 10♠, K♣])` is `[10♠, K♣, K♥]`, and `best(&[K♥, 10♠, K♣])` is `Some(K♥)`.
- `distinct(&[A♠, A♠, 2♣])` is `2`.
- `Rank::Num(10) < Rank::Jack` is `true`, and `Card::default()` is `A♠`.
