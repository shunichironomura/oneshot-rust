`std::future::poll_fn` は、クロージャ `FnMut(&mut Context<'_>) -> Poll<T>` を、`poll` のたびにその
クロージャを呼ぶ Future に変えます。`move` を付けたクロージャが join の状態、つまり入力と、入力ごとに
完了時の出力を受け取る `Option` を所有します。`None` は「まだ実行中」という意味も兼ねていて、これで
完了した入力を再びポーリングしないようにしています。ポーリングのたびに、まず実行中の入力をポーリングし、
その後で出力がすべてそろったかを調べます。

最初の提出でコンパイルを通すには、4 つの点に注意が必要です。

- **ピン留め。** `Future::poll` は `self: Pin<&mut Self>` を受け取ります。`Pin::new(fut)` が
  使えるのは `F: Unpin` のときだけです（「`F` cannot be unpinned」、E0277）。入力の型は型引数で、
  テストでは `Unpin` でない `async` ブロックです。シグネチャに `F: Unpin` を足しても、エラーが
  ハーネス側に移るだけです。`Box::pin` は各入力をヒープに移し、そこから二度と動かしません。
  `fut.as_mut()` で `Pin<Box<F>>` を `poll` が受け取る `Pin<&mut F>` として借用し直せます。
  `Pin<Box<F>>` を（`Vec` の中へ、クロージャの中へ、その後は結合した Future ごと）ムーブしても、
  動くのはポインタだけで、入力そのものは動きません。`Pin<Box<F>>` は `Unpin` です。
- **`move`。** これがないと、クロージャは `join_all` のローカル変数を借用しますが、それらは
  `join_all` から戻るときに破棄されます。コンパイラは「the parameter type `F` may not live long
  enough」と報告して `F: 'static` を提案し、さらに `futs` の寿命が足りない（does not live long
  enough）とも報告します。この提案には従わないでください。ハーネスの入力はローカル変数を借用しています。
- **出力の取り出し。** `poll_fn` はポーリングのたびにクロージャを呼ぶので、クロージャは `FnMut` で、
  キャプチャした変数をムーブして取り出せません。`outs.into_iter()` は「cannot move out of `outs`,
  a captured variable in an `FnMut` closure」（E0507）で拒否されます。`Option::take` なら各出力を
  ムーブして取り出し、代わりに `None` を残します。
- **`Clone` は使えない。** `vec![None; n]` は要素を複製するので、`F::Output: Clone` が必要です。
  `None` のベクタは `map(|_| None)` で作ります。

動作にも落とし穴があります。

- **実行中の入力はすべてポーリングする。** 出力がそろったかの最後の判定に `all` を使うのは問題ありません。
  しかし `iter_mut().all(...)` の中でポーリングすると、短絡評価なので `Pending` を返した最初の
  入力で止まります。途中で `return Poll::Pending` しても同じです。ループで各入力を `.await` する
  `async move` ブロックはコンパイルが通りますが、入力を 1 つずつ順に実行します。1 つ目の例では、
  このような `join_all` は `a0`、`a1`、`b0`、`b1`、`b2` の順にログを記録し、ポーリングが 4 回かかります。
- **先にポーリングしてから判定する。** 出力がそろったかを先に調べてからポーリングするクロージャは、最後の入力が
  完了したポーリングで `Pending` を返してしまいます。そのポーリングの間には誰もウェイカーを起こしていないので、
  実際のエグゼキュータは結合した Future を二度とポーリングしません。
- **`cx` をそのまま渡す。** 入力は、ポーリングされたときの `Context` のウェイカーを起こします。エグゼキュータは
  それによって、結合した Future が先に進めるようになったことを知ります。`Waker::noop()` のような
  自前のウェイカーで入力をポーリングすると、その通知が失われます。結合した Future 自身が `cx.waker()` を
  起こす必要はなく、問題文でも禁じています。起こすとエグゼキュータが無駄にポーリングし直すことになります。
- **順序を保つ。** 出力は入力の順に並べます。入力が完了した順ではありません。完了した入力を
  `swap_remove` で取り除くのも誤りです。最後の入力が空いた場所に移るので、残りの入力が添字の順に
  ポーリングされなくなります。

### 別解

`Future` を自分で実装する構造体でも書けますが、1 つ注意点があります。`poll` が受け取るのは
`Pin<&mut Self>` で、フィールドを書き換えるには `&mut Self` が必要です。それを `get_mut`
（や `DerefMut`）で得られるのは `Self: Unpin` のときだけです。`Unpin` のような自動トレイトは、
すべてのフィールドが実装しているときだけ型に実装されます。`Pin<Box<F>>` は常に `Unpin` ですが、
`F::Output` はそうとは限らないので、そのままでは `get_mut` のところで「`<F as Future>::Output`
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

struct JoinAll<F: Future> {
    slots: Vec<Slot<F>>,
}

impl<F: Future> Future for JoinAll<F> {
    type Output = Vec<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // `get_mut` needs `JoinAll<F>: Unpin`, which holds because every slot is `Unpin`.
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
