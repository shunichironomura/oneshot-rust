書く必要があるメソッドは `next` だけです。`take`・`sum`・`collect`・`count`・`max` などのアダプタは、
`Iterator` が `next` をもとに提供するメソッドです。

難しいのは数列の終わり方です。次に返す値を `Option<u64>` で持つと、3 つの場合をまとめて扱えます。
空の数列は `None`、`1` を出した後も `None` で、`self.next?` は何も残っていなければその場で `None` を返します。
`u64` のまま `0` を「終わり」とみなしても動きますが、`Option` のほうが意味がはっきりします。

### 別解

`std::iter::successors` を使うと、最初の値と次の値を求める関数から同じ数列を作れます。
ただし与えられた `Collatz` 型には合わないので、包んで使います。

```rust
pub struct Collatz {
    inner: Box<dyn Iterator<Item = u64>>,
}

impl Collatz {
    pub fn new(start: u64) -> Self {
        let first = (start != 0).then_some(start);
        let inner = std::iter::successors(first, |&n| match n {
            1 => None,
            n if n % 2 == 0 => Some(n / 2),
            n => Some(3 * n + 1),
        });
        Collatz { inner: Box::new(inner) }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
```
