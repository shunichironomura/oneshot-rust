エディタには、トランプのカードを表す 3 つの型 `Suit`・`Rank`・`Card` がすでに入っています。
どの型も `Clone`、`Copy`、`Debug`、`Default`、`PartialEq`、`Eq`、`PartialOrd`、`Ord`、`Hash` を
derive しています。すべてが下の規則に従うようにして、3 つの関数を実装してください。

```rust
pub fn sorted(cards: &[Card]) -> Vec<Card>
pub fn best(cards: &[Card]) -> Option<Card>
pub fn distinct(cards: &[Card]) -> usize
```

- `Suit` には 4 つのバリアントがあり、`Clubs` < `Diamonds` < `Hearts` < `Spades` の順です。
- `Rank` のバリアントは `Num(u8)`、`Jack`、`Queen`、`King`、`Ace` です。`Num(n)` は数字 `n` の
  カードで、`n` は 2 以上 10 以下です（テストではほかの数は使いません）。数字のカードどうしは `n` の大小で
  比べ、どの数字のカードも `Jack` より下です。`Jack` < `Queen` < `King` < `Ace` です。
- `Card` には公開フィールド `rank: Rank` と `suit: Suit` があります。カードはまず `rank` で比べ、
  `rank` が同じなら `suit` で比べます。2 枚のカードは、両方のフィールドが等しいときに等しいとします。
- `Suit::default()` は `Spades`、`Rank::default()` は `Ace`、`Card::default()` はスペードのエースです。
- これらの順序と既定値は、型そのものについて成り立たなければなりません。テストでは 3 つの関数を
  呼ぶだけでなく、値を `<`、`>`、`partial_cmp`、`cmp` で比べたり、`sort()` や `max()` を呼んだり、
  カードを `BTreeSet` や `HashSet` に入れたりします。
- `sorted` は、すべてのカード（重複も含む）を昇順に並べて返します。`best` は最も大きいカードを返し、
  `cards` が空なら `None` を返します。`distinct` は異なるカードの枚数を返します。
- 型の定義は変更して構いません（derive を書き換える、トレイトを手で実装する、など）。ただし、型・
  バリアント・フィールドの名前は変えず、フィールドは `pub` のままにし、9 つのトレイトはすべて
  実装されたままにしてください。

### 例

以下では、`K♥` は `Card { rank: Rank::King, suit: Suit::Hearts }`、`10♠` は
`Card { rank: Rank::Num(10), suit: Suit::Spades }` を表します（ほかも同様）。

- `sorted(&[K♥, 10♠, K♣])` は `[10♠, K♣, K♥]`、`best(&[K♥, 10♠, K♣])` は `Some(K♥)` です。
- `distinct(&[A♠, A♠, 2♣])` は `2` です。
- `Rank::Num(10) < Rank::Jack` は `true`、`Card::default()` は `A♠` です。
