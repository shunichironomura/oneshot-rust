標準ライブラリには `Future` はありますが、`Iterator` の非同期版にあたる `Stream` はありません。
小さなストリームのライブラリを一から書いてください。下の骨組みはエディタに入っています。
`Stream` トレイトと `next` のシグネチャは与えられています。`Iter` と `Zip` にはすでにフィールドがありますが、
変更して構いません。

```rust
pub trait Stream {
    type Item;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}

pub trait StreamExt: Stream {
    fn next(&mut self) -> impl Future<Output = Option<Self::Item>> + '_
    where
        Self: Unpin,
    { /* 実装する */ }
}

impl<S: Stream + ?Sized> StreamExt for S {}

pub struct Iter<I> { /* フィールド */ }
pub fn iter<I: IntoIterator>(items: I) -> Iter<I::IntoIter>;
impl<I: Iterator> Stream for Iter<I> { type Item = I::Item; /* poll_next */ }

pub struct Zip<A: Stream, B: Stream> { /* フィールド */ }
pub fn zip<A: Stream + Unpin, B: Stream + Unpin>(a: A, b: B) -> Zip<A, B>;
impl<A: Stream + Unpin, B: Stream + Unpin> Stream for Zip<A, B> {
    type Item = (A::Item, B::Item);
    /* poll_next */
}
```

- `poll_next` は次の要素か、終わりなら `Ready(None)` を返し、まだなければ `cx` を通してタスクが
  起こされるようにしてから `Pending` を返し、`Ready(None)` の後に再びポーリングされることはありません。
- `s.next()` は Future を返します。この Future は `s` の次の要素に解決し、`s` が終わっていれば `None` に
  解決します。ポーリングされるたびに、同じ `cx` で `s` を 1 回ポーリングし、`s` が `Pending` を返したら
  `Pending` を返します。`next` は `Unpin` なストリームならどれにでも呼べます。
  `dyn Stream<Item = u32> + Unpin` のようなトレイトオブジェクトにも呼べます。
- `iter(items)` は `items` の要素を順に返すストリームを返します。`Pending` になることはありません。
- `zip(a, b)` は組のストリームを返します。`a` の 1 番目の要素と `b` の 1 番目の要素、
  次に 2 番目どうし、という順に組にします。ポーリングされるたびに、次のように動きます。
  1. `a` の要素を持っていなければ、`a` を 1 回ポーリングします。`a` が `Pending` を返したら、`b` をポーリングせずに
     `Pending` を返します。`a` が終わっていたら `Ready(None)` を返します。それ以外の場合は、
     その `a` の要素を持ちます。
  2. `b` を 1 回ポーリングします。`b` が `Pending` を返したら `Pending` を返し、`a` の要素は次のポーリングのために
     持ったままにします。`b` が終わっていたら `Ready(None)` を返します。それ以外の場合は 2 つの要素の組を返し、
     要素を持っていない状態に戻ります。

  `zip` は受け取った `cx` をそのまま `a` と `b` に渡し、自分では何も起こしません。
- トレイトの定義、シグネチャ、`impl` の境界は与えられたまま（`iter` はどんなイテレータも、`zip` は
  `Unpin` なストリームならどの 2 つも受け付ける）にし、型、`impl` ブロック、関数は自由に追加して構いません。

テストハーネスは独自のエグゼキュータで Future を動かします。このエグゼキュータは、Future が起こされてからでないと
再びポーリングしません。ハーネスは独自のストリームも使い、各ストリームがポーリングされた回数を数えます。
たとえば `slow(items)` は、各要素の前と終わりの前に（タスクを起こしてから）`Pending` を返します。
つまり `Pending`、1 番目の要素、`Pending`、2 番目の要素、……、`Pending`、`Ready(None)` の順です。

### 例

- `iter(vec![1, 2, 3])` を終わるまでポーリングすると、`Ready(Some(1))`、`Ready(Some(2))`、`Ready(Some(3))`、
  `Ready(None)` の順に返ります。
- `let mut s = iter(["x"]);` の後、`s.next().await` は `Some("x")`、その次は `None` です。
- `zip(iter(1..=3), slow(vec!['a', 'b']))` を終わるまでポーリングすると、`Pending`、`Ready(Some((1, 'a')))`、
  `Pending`、`Ready(Some((2, 'b')))`、`Pending`、`Ready(None)` の順に返ります。`iter` のストリームは 3 回
  （要素 `3` は組になりません）、`slow` は 6 回ポーリングされます。
