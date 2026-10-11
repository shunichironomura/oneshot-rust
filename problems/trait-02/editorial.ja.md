`impl<I> Pairs for I where I: Iterator, I::Item: Clone` は**ブランケット実装**です。この 1 つの `impl` で、
`Pairs` がスコープにあれば `vec::IntoIter`、`Chars`、範囲など、あらゆるイテレータに `pairs` が生えます。
`pairs` は `self` を `PairsIter` に包むだけで、実際の処理は `next` の中で遅延して行います。

直前の組の 2 番目の要素は、次の組の 1 番目の要素にもなります。そのため `PairsIter` はこの要素を覚えておく必要があります。
型は `Option<I::Item>` です。要素の型は `I` の関連型なので、構造体に型引数を追加する必要はありません。
1 つは返す組に入れ、もう 1 つは手元に残すので、`I::Item: Clone` が必要になります。内側のイテレータが尽きたら、
`?` でその場で終わります。

### 別解

`Peekable` を使えば、1 つ先の要素を覗けます。元のイテレータの代わりに `Peekable` を持ち、覗いた要素を複製します。

```rust
use std::iter::Peekable;

pub trait Pairs: Iterator + Sized {
    fn pairs(self) -> PairsIter<Self>;
}

pub struct PairsIter<I: Iterator> {
    iter: Peekable<I>,
}

impl<I> Pairs for I
where
    I: Iterator,
    I::Item: Clone,
{
    fn pairs(self) -> PairsIter<I> {
        PairsIter {
            iter: self.peekable(),
        }
    }
}

impl<I> Iterator for PairsIter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.iter.next()?;
        let second = self.iter.peek()?.clone();
        Some((first, second))
    }
}
```
