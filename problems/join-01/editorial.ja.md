`std::future::poll_fn` は、クロージャ `FnMut(&mut Context<'_>) -> Poll<T>` を、`poll` のたびにその
クロージャを呼ぶ Future に変えます。`move` を付けたクロージャが join の状態、つまり入力と、入力ごとに
完了時の出力を受け取る `Option` を所有します。`None` は「まだ実行中」という意味も兼ねていて、これで
完了した入力を再び poll しないようにしています。poll のたびに、まず実行中の入力を poll し
（`poll_input`）、その後で出力がすべてそろったかを調べます。

最初の提出でコンパイルを通すには、4 つの点に注意が必要です。

- **ピン留め。** `Future::poll` は `self: Pin<&mut Self>` を受け取ります。`Pin::new(&mut a)` が
  使えるのは `A: Unpin` のときだけです（「`A` cannot be unpinned」、E0277）。入力の型は型引数で、
  テストでは `Unpin` でない `async` ブロックです。シグネチャに `A: Unpin` を足しても、エラーが
  ハーネス側に移るだけです。`Box::pin(a)` は入力をヒープに移し、そこから二度と動かしません。
  `a.as_mut()` で `Pin<Box<A>>` を `poll` が受け取る `Pin<&mut A>` として借用し直せます。
  `Pin<Box<A>>` 自体は `Unpin` なので、クロージャが所有できます。
- **`move`。** これがないと、クロージャは `join` のローカル変数を借用しますが、それらは `join` から
  戻るときに破棄されます。コンパイラは `a` の寿命が足りない（does not live long enough）と報告し、
  さらに「the parameter type `A` may not live long enough」として `A: 'static` を提案します。
  この提案には従わないでください。ハーネスの入力はローカル変数を借用しています。
- **出力の取り出し。** `poll_fn` は poll のたびにクロージャを呼ぶので、クロージャは `FnMut` で、
  キャプチャした変数をムーブして取り出せません。`a_out.unwrap()` は「cannot move out of `a_out`,
  a captured variable in an `FnMut` closure」（E0507）で拒否されます。`Option::take` なら出力を
  ムーブして取り出し、代わりに `None` を残します。
- **`Clone` は使えない。** `vec![None; n]` は要素を複製するので、`F::Output: Clone` が必要です。
  `None` のベクタは `map(|_| None)` で作ります。

動作にも落とし穴があります。

- **実行中の入力はすべて poll する。** 出力がそろったかの最後の判定に `&&` を使うのは問題ありません。
  しかし `poll_a() && poll_b()` や `iter_mut().all(...)` は短絡評価なので、`Pending` を返した最初の
  入力で止まります。途中で `return Poll::Pending` しても同じです。`async move { (a.await, b.await) }`
  はコンパイルが通りますが、入力を 1 つずつ順に実行します。1 つ目の例では、どれも `a0`、`a1`、`b0`、
  `b1`、`b2` の順にログを記録し、poll が 4 回かかります。
- **先に poll してから判定する。** 出力がそろったかを先に調べてから poll するクロージャは、最後の入力が
  完了した poll で `Pending` を返してしまいます。その poll の間には誰も waker を起こしていないので、
  実際の実行器は結合した Future を二度と poll しません。
- **`cx` をそのまま渡す。** 入力は、poll されたときの `Context` の waker を起こします。実行器は
  それによって、結合した Future が先に進めるようになったことを知ります。`Waker::noop()` のような
  自前の waker で入力を poll すると、その通知が失われます。結合した Future 自身が `cx.waker()` を
  起こす必要はなく、起こすと実行器が無駄に poll し直すことになります。
- **順序を保つ。** 出力は入力の順に並べます。入力が完了した順ではありません。完了した入力を
  `swap_remove` で取り除くのも誤りです。最後の入力が空いた場所に移るので、残りの入力が添字の順に
  poll されなくなります。

### 別解

`Future` を自分で実装する構造体でも書けますが、1 つ注意点があります。`poll` が受け取るのは
`Pin<&mut Self>` で、フィールドを書き換えるには `&mut Self` が必要です。それを `get_mut`
（や `DerefMut`）で得られるのは `Self: Unpin` のときだけです。`Unpin` のような自動トレイトは、
すべてのフィールドが実装しているときだけ型に実装されます。`Pin<Box<F>>` は常に `Unpin` ですが、
`F::Output` はそうとは限らないので、そのままでは `get_mut` のところで「`<A as Future>::Output`
cannot be unpinned」と報告されます。この構造体は、自分の中に直接持っているものを何もピン留めしません
（入力は `Pin<Box<_>>` の先にあり、出力は保持するだけです）。そのため、出力の型にかかわらず `Unpin` を
約束してかまいません。`Unpin` の実装は安全なコードで書けます。`unsafe` が必要になるのは、ピン留めを
フィールドに伝える（ピンの射影）場合だけです。

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

/// One input of a join: running (pinned on the heap), then completed with its output.
enum Slot<F: Future> {
    Running(Pin<Box<F>>),
    Done(F::Output),
    Taken,
}

// The future is behind a `Box` and the output is never pinned, so a `Slot` may move freely
// even if `F::Output` is not `Unpin`.
impl<F: Future> Unpin for Slot<F> {}

impl<F: Future> Slot<F> {
    fn new(fut: F) -> Self {
        Slot::Running(Box::pin(fut))
    }

    /// Polls the input unless it has completed. Returns whether its output is available.
    fn poll(&mut self, cx: &mut Context<'_>) -> bool {
        if let Slot::Running(fut) = self {
            match fut.as_mut().poll(cx) {
                Poll::Ready(out) => *self = Slot::Done(out),
                Poll::Pending => return false,
            }
        }
        true
    }

    fn take(&mut self) -> F::Output {
        match std::mem::replace(self, Slot::Taken) {
            Slot::Done(out) => out,
            _ => unreachable!("the input has not completed"),
        }
    }
}

struct Join<A: Future, B: Future> {
    a: Slot<A>,
    b: Slot<B>,
}

impl<A: Future, B: Future> Future for Join<A, B> {
    type Output = (A::Output, B::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // `get_mut` needs `Join<A, B>: Unpin`, which holds because both slots are `Unpin`.
        let this = self.get_mut();
        let a_done = this.a.poll(cx);
        let b_done = this.b.poll(cx);
        if a_done && b_done { Poll::Ready((this.a.take(), this.b.take())) } else { Poll::Pending }
    }
}

pub fn join<A: Future, B: Future>(a: A, b: B) -> impl Future<Output = (A::Output, B::Output)> {
    Join { a: Slot::new(a), b: Slot::new(b) }
}

struct JoinAll<F: Future> {
    slots: Vec<Slot<F>>,
}

impl<F: Future> Future for JoinAll<F> {
    type Output = Vec<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let mut all_done = true;
        for slot in &mut this.slots {
            all_done &= slot.poll(cx);
        }
        if all_done {
            Poll::Ready(this.slots.iter_mut().map(Slot::take).collect())
        } else {
            Poll::Pending
        }
    }
}

pub fn join_all<F: Future>(futs: Vec<F>) -> impl Future<Output = Vec<F::Output>> {
    JoinAll { slots: futs.into_iter().map(Slot::new).collect() }
}
```

`join` は、ヒープを使わずに入力をピン留めすることもできます。`async` ブロックは実行中ピン留めされて
いるので、ブロック自身の中に置いた値なら `pin!` でピン留めできます。クロージャは出力を借用するだけに
なり、`.await` の後（クロージャがなくなった後）なら `unwrap` で出力をムーブして取り出せます。模範解答の
`join` をこれに置き換えます（`use std::pin::pin;` を追加します）。`join_all` には使えません。`pin!` が
ピン留めするのは 1 つの値で、`Vec` の要素を `unsafe` なしでその場でピン留めすることはできないからです。

```rust
pub fn join<A: Future, B: Future>(a: A, b: B) -> impl Future<Output = (A::Output, B::Output)> {
    async move {
        // The `async` block is pinned while it runs, so it can pin its own locals.
        let mut a = pin!(a);
        let mut b = pin!(b);
        let mut a_out = None;
        let mut b_out = None;
        poll_fn(|cx| {
            if a_out.is_none()
                && let Poll::Ready(out) = a.as_mut().poll(cx)
            {
                a_out = Some(out);
            }
            if b_out.is_none()
                && let Poll::Ready(out) = b.as_mut().poll(cx)
            {
                b_out = Some(out);
            }
            if a_out.is_some() && b_out.is_some() { Poll::Ready(()) } else { Poll::Pending }
        })
        .await;
        // The closure and its borrows are gone, so the outputs can be moved out.
        (a_out.unwrap(), b_out.unwrap())
    }
}
```

`join_all` は、完了した入力をすぐに破棄することもできます。`retain_mut` は実行中の入力を順序を保った
まま残します。出力は入力の添字と組にしておき、最後に添字で並べ替えます。`Option` は要らず、集めた出力は
`std::mem::take` で `FnMut` のクロージャから取り出します。模範解答の `join_all` をこれに置き換えます。

```rust
pub fn join_all<F: Future>(futs: Vec<F>) -> impl Future<Output = Vec<F::Output>> {
    let mut running: Vec<(usize, Pin<Box<F>>)> =
        futs.into_iter().map(Box::pin).enumerate().collect();
    let mut done: Vec<(usize, F::Output)> = Vec::new();
    poll_fn(move |cx| {
        // `retain_mut` keeps the remaining inputs in order and drops the completed ones.
        running.retain_mut(|(index, fut)| match fut.as_mut().poll(cx) {
            Poll::Ready(out) => {
                done.push((*index, out));
                false
            }
            Poll::Pending => true,
        });
        if running.is_empty() {
            let mut done = std::mem::take(&mut done);
            done.sort_by_key(|&(index, _)| index);
            Poll::Ready(done.into_iter().map(|(_, out)| out).collect())
        } else {
            Poll::Pending
        }
    })
}
```
