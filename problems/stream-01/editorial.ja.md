**`next`。** `std::future::poll_fn` は、クロージャ `FnMut(&mut Context<'_>) -> Poll<T>` を Future に変えます。
`next` に必要なのはこれだけで、Future がポーリングされるたびにストリームを 1 回ポーリングします。
コンパイルを通すには 2 つの点に注意が必要です。

- `poll_next` は `Pin<&mut Self>` を受け取ります。メソッド呼び出しは `&mut Self` を自動で `Pin` に
  包んではくれないので、`self.poll_next(cx)` はエラーです。`Pin` は `Pin::new` で作れますが、
  `Self: Unpin` のときに限ります。`next` がこれを要求しているのはそのためです。
- `Pin::new(self)` と書くと、`self`（`Copy` でない `&mut Self`）が最初の呼び出しにムーブされ、
  クロージャは 1 回しか呼べなくなります。`poll_fn` が求めるのは `FnMut` なのでエラーになります（E0507）。
  `Pin::new(&mut *self)` なら、呼び出しのたびに再借用するだけで済みます。Future はストリームを借用し続けます。
  返り値の型の `+ '_` はそのことを表しています。

**`Iter` と `Zip` の `Unpin`。** `self: Pin<&mut Self>` のフィールドを変更するには `get_mut`
（または `mut self` にして `DerefMut`）が必要で、どちらも `Self: Unpin` を要求します。`Unpin` は自動トレイトです。
`Iter<I>` が `Unpin` になるのは `I` が `Unpin` のときだけで、`Zip<A, B>` は `a` の要素を保持するので、
`A`、`B` に加えて `A::Item` も `Unpin` のときだけです。impl ブロックは `I` や `A::Item` について何も約束していないので、
`self.get_mut()` は「`I` cannot be unpinned」「`<A as Stream>::Item` cannot be unpinned」というエラーになります。
コンパイラは `I: Unpin` や `<A as Stream>::Item: Unpin` を追加するよう提案しますが、それでは与えられた境界が
変わってしまいます。ハーネスは `Unpin` でないイテレータや、`async` ブロックの Future（これも `Unpin` ではありません）
を要素とするストリームも使います。

代わりに、`Iter` や `Zip` をピン留めしても中身はピン留めされない、と宣言します。

```rust
impl<I> Unpin for Iter<I> {}
impl<A: Stream, B: Stream> Unpin for Zip<A, B> {}
```

`Unpin` は安全なトレイトです。このコードは `Pin<&mut I>` も `Pin<&mut A::Item>` も作らない
（「構造的ピン留め」をしない）ので、この impl は正しいものです。イテレータは普通の `&mut I` で使い、
保持する要素はムーブして出し入れするだけです。`a` と `b` は `Pin` を通してポーリングしますが、
`Pin::new(&mut this.a)` に必要な `A: Unpin` は impl ブロックが保証しています。`futures` クレートのストリームも
同じようにしています。impl には構造体の境界を書く必要があります。`Zip<A, B>` には `A: Stream` と
`B: Stream` が必要なので、`impl<A, B> Unpin for Zip<A, B> {}` はエラーになります。

**`zip` で要素を持っておく。** `a` の要素を取り出したら、`b` が `Pending` を返してもその要素を失ってはいけません。
つい書きたくなる `match ready!(Pin::new(&mut this.b).poll_next(cx)) { ... }` は、`b` が `Pending` のときに
`poll_next` から戻り、`a` の要素を捨ててしまいます。すると次のポーリングでは、`a` の*次の*要素が `b` の要素と組になります。
模範解答のように、`Pending` を返す前に要素を `held` に戻してください。要素を持っている間は `a` をポーリングせず、
要素を持つまでは `b` をポーリングしないことも大切です。ハーネスはポーリングの回数を数えています。

### 別解

`next` は、`poll_fn` の代わりに手で書いた Future を返すこともできます。トレイトの中の `Self` はサイズが
不定かもしれない（`next` を `dyn Stream` に使えるように、ブランケット impl が `?Sized` にしている）ので、
構造体もサイズ不定のストリームを受け付ける必要があります。`S: ?Sized` がないと、`Next { stream: self }` は
コンパイルできません（「the size for values of type `Self` cannot be known」）。`Next` は参照を持つだけなので
`Unpin` であり、`DerefMut` を通して `self.stream` に触れます。

```rust
pub trait StreamExt: Stream {
    fn next(&mut self) -> impl Future<Output = Option<Self::Item>> + '_
    where
        Self: Unpin,
    {
        Next { stream: self }
    }
}

struct Next<'a, S: ?Sized> {
    stream: &'a mut S,
}

impl<S: Stream + Unpin + ?Sized> Future for Next<'_, S> {
    type Output = Option<S::Item>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut *self.stream).poll_next(cx)
    }
}
```

`impl Unpin` を書く代わりに、`Unpin` でないかもしれないものを `Box` に入れる方法もあります。`Box<T>` は
どんな `T` でも `Unpin` です。Box をムーブしても中身はムーブされないからです。ただし `Iter` ごと、
保持する要素ごとにメモリ確保が必要になります。

```rust
pub struct Iter<I> {
    iter: Box<I>,
}

pub fn iter<I: IntoIterator>(items: I) -> Iter<I::IntoIter> {
    Iter { iter: Box::new(items.into_iter()) }
}

pub struct Zip<A: Stream, B: Stream> {
    a: A,
    b: B,
    held: Option<Box<A::Item>>,
}
```

このとき `Zip::poll_next` では、保持している要素を `Some(item) => *item` で取り出し、
`this.held = Some(Box::new(a_item))` で戻します。
