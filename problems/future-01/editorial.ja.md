`poll` が受け取るのは `&mut self` ではなく `self: Pin<&mut Self>` です。フィールドを読むときは `Deref` を通りますが、
代入するときは `DerefMut` を通ります。`Pin<&mut T>` が `DerefMut` を実装するのは `T: Unpin` のときだけです。
`Countdown` は `u32` しか持たないので、自動的に `Unpin` になります。ただし `deref_mut` は `Pin` 自体を可変で
借用するので、引数を `mut self` と宣言する必要があります。宣言しないと、`self.remaining -= 1` が
「cannot borrow `self` as mutable」でエラーになります（`self.get_mut()` を使う方法もあります。下の別解を見てください）。

`Pending` を返す Future は、もう一度ポーリングする価値があるときにタスクが起こされることを約束します。
実際のエグゼキュータは、それまで眠っています。`Countdown` には待つものがないので、現在のタスクをすぐに起こします。
起こすときは、必ずそのとき渡された `Context` のウェイカーを使ってください。次のポーリングでは別のウェイカーが
渡されるかもしれません。

`block_on` はまず `into_future()` を呼びます。`.await` も同じことをするので、`Delay::new(1).await` と書けます。
また、どの `Future` も `IntoFuture` なので、`block_on` はどちらも受け付けます。ポーリングには `Pin<&mut F>` が必要で、
`Pin::new(&mut fut)` には `F: Unpin` が要るので、ジェネリックな `F` ではコンパイルエラー
（「`F` cannot be unpinned」）になります。`F: Unpin` の境界を足しても解決しません。`async` ブロックは
`Unpin` ではない（`.await` をまたいで自分のローカル変数への参照を持つことがある）ので、`async` ブロックを
渡すテストがコンパイルできなくなります。代わりに `std::pin::pin!` を使うと、値をその場（スタック上）で
固定できます。

`Future::poll` は `Pin<&mut F>` を値で受け取るので、ループの中で `fut.poll(&mut cx)` と書くと、最初の繰り返しで
`fut` がムーブされてしまいます。`fut.as_mut()` を使えば、1 回の呼び出しの間だけ借用し直せます。
`Waker::noop()`（Rust 1.85）は何もしない `&'static Waker` で、`Context::from_waker` で `Context` にできます。

### 別解

`Pin::get_mut`（これも `Unpin` な型でだけ使えます）を使うと、ピンを普通の `&mut Self` に戻せます。

```rust
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
    let this = self.get_mut();
    if this.remaining == 0 {
        return Poll::Ready(this.total);
    }
    this.remaining -= 1;
    cx.waker().wake_by_ref();
    Poll::Pending
}
```

`Box::pin` を使うと、Future をヒープ上で固定できます。また `Waker::noop` ができる前は、`Wake` を実装した型の
`Arc` から何もしないウェイカーを作っていました。

```rust
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

struct NoopWaker;

impl Wake for NoopWaker {
    fn wake(self: Arc<Self>) {}
}

pub fn poll_count<F: Future>(fut: F) -> (F::Output, usize) {
    let mut fut = Box::pin(fut);
    let waker = Waker::from(Arc::new(NoopWaker));
    let mut cx = Context::from_waker(&waker);
    let mut polls = 0;
    loop {
        polls += 1;
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(output) => return (output, polls),
            Poll::Pending => {}
        }
    }
}
```

ループを `block_on` に置き、`poll_count` ではポーリング回数を数える別の Future で包む方法もあります。
ジェネリックな `F` は `Unpin` とは限りませんが、`Pin<Box<F>>` は常に `Unpin` です。そのため包んだ Future も
`Unpin` になり、`Countdown` と同じように `DerefMut` を通してカウンタを更新できます。

```rust
pub fn block_on<F: IntoFuture>(f: F) -> F::Output {
    let mut fut = pin!(f.into_future());
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
            return output;
        }
    }
}

struct Counted<F> {
    fut: Pin<Box<F>>,
    polls: usize,
}

impl<F: Future> Future for Counted<F> {
    type Output = (F::Output, usize);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.polls += 1;
        let polls = self.polls;
        self.fut.as_mut().poll(cx).map(|output| (output, polls))
    }
}

pub fn poll_count<F: Future>(fut: F) -> (F::Output, usize) {
    block_on(Counted { fut: Box::pin(fut), polls: 0 })
}
```
