`#[derive(PartialOrd, Ord)]` はカードのことを何も知りません。従うのは**宣言した順番**です。

- 列挙型では、先に宣言したバリアントのほうが小さくなります。同じバリアントどうしは中身のフィールドで
  比べるので、`Num(9) < Num(10)` です。
- 構造体では、フィールドを宣言した順に 1 つずつ比べます（タプルと同じです）。最初のフィールドで決まり、
  等しいときだけ次のフィールドを見ます。

テンプレートはスートもランクも強いほうから並べていたので、derive した比較はすべて逆になっていました。
最後に宣言した `Num(u8)` は `Ace` よりも強くなっていました。さらに `Card { suit, rank }` では、ランクより先に
スートを比べていました。バリアントを弱いほうから強いほうへ宣言し直し、`rank` を `suit` より前に置けば、
比較のコードを 1 行も書かずにすべて直ります。`PartialEq`・`Eq`・`Hash` は順番に依存しないので、
`HashSet<Card>` はどちらでも正しく動いていました。

`#[default]` は位置ではなくバリアントに付ける印なので、`Spades` や `Ace` と一緒に動かす必要があります。
並びを逆にしたときに先頭の行に残してしまうと、既定のスートが `Clubs` になりますし、`#[default] Num(u8)` は
コンパイルすら通りません。既定値にできるのはユニットバリアント（フィールドのないバリアント）だけだからです。
`Card` の derive した `Default` は各フィールドの既定値を使うので、スペードのエースになります。

型が正しければ、関数は短く書けます。`sort()`・`max()`・`HashSet` はどれも derive したトレイトを使います。
`max()` が返すのは `Option<&Card>` なので、`copied()` で `Option<Card>` にします。

### 別解

宣言はそのままにして、順序を手で実装する方法です。この場合は `PartialOrd` と `Ord` の両方を derive から外し、
`partial_cmp` は `Some(self.cmp(other))` を返すだけにします。手書きの `Ord` と一緒に
`#[derive(PartialOrd)]` を残してもコンパイルは通りますが、`<` と `cmp` の結果が食い違ってしまいます。
標準ライブラリはどちらを使うこともあり、たとえば今の `sort()` や `max()` は `<` で比べるので、手書きの `Ord` は
まったく使われません。

```rust
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)] // PartialOrd, Ord は外す
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

// `Card` も derive から PartialOrd と Ord を外す。
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

`Suit` にも同じように `value` と 2 つの impl を書きます。なお、`Rank::value` は `n` が 10 以下であることを
前提にしています。`Num(11)` があると、`Jack` と `==` ではないのに比較結果は `Equal` になり、`Ord` の約束を
破ってしまいます。derive した順序には、このような落とし穴はありません。

`distinct` は、カードを並べ替えてから重複を取り除いても求められます。`dedup` が取り除くのは**連続した**
重複だけなので、先に並べ替えてください。

```rust
pub fn distinct(cards: &[Card]) -> usize {
    let mut cards = sorted(cards);
    cards.dedup();
    cards.len()
}
```
