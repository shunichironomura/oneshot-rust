`poll` が受け取るのは `self: Pin<&mut Self>` ですが、フィールド（予算や、破棄するときの内側の Future）を
変更するには `&mut Self` が必要です。`Pin` がそれを渡してくれるのは `Self: Unpin` のときだけです
（`get_mut` や、`mut self` にしたときの `DerefMut` で）。`Unpin` は自動トレイトで、構造体はすべてのフィールドが
`Unpin` のときに `Unpin` になります。`inner: Option<F>` のように内側の Future を直接持つと、`Budget<F>` が
`Unpin` になるのは `F` が `Unpin` のときだけで、ハーネスの `async` ブロックや `ticks` の Future は `Unpin` では
ありません。そのため、`self.inner = None` は「cannot assign to data in dereference of `Pin<&mut Budget<F>>`」
（E0594）で、`self.get_mut()` や内側の Future への `Pin::new(&mut ...)` は「`F` cannot be unpinned」（E0277）で
拒否されます。コンパイラの提案どおり `F: Unpin` を付けても、エラーがハーネス側に移るだけです。

`F` を直接持ったままにするには、*ピンの射影*（pin projection）が必要です。`Pin<&mut Budget<F>>` を
フィールドの `Pin<&mut F>` に変える操作で、`unsafe`（`Pin::map_unchecked_mut` や `pin-project` などの
クレート）を使いますが、この問題では禁止されています。安全な方法は、内側の Future をヒープ上に固定する
ことです。`Box::pin(fut)` が返す `Pin<Box<F>>` は、`F` が何であっても `Unpin` です。box を動かしても
動くのはポインタだけで、Future は動かないからです。フィールドを `Option<Pin<Box<F>>>` と `u32` にすれば、
`Budget<F>` は自動的に `Unpin` になり、`get_mut` で普通の `&mut Budget<F>` が得られます。`fut.as_mut()` で、
box を `poll` が受け取る `Pin<&mut F>` として借用し直せます。

`Option` にしておくと、`Budget` は内側の Future を早めに破棄できます。`None` を代入すると、その場で（`poll` の
中で）box と中の Future が破棄されます。Rust では、Future を破棄することがキャンセルです。`.await` の途中で
破棄された `async` ブロックは二度と再開されず、その時点で持っている値（待っている Future など）が破棄
されます。内側の Future は、`Ready` を返した後でも破棄することに意味があります。完了した `async` ブロックは
ローカル変数やキャプチャをすでに破棄していますが、ハーネスの `ticks` のような手書きの Future は、破棄される
まで自分の持ち物を持ち続けるからです。

回数は慎重に数えましょう。予算 `0` は「一度も poll しない」という意味なので、poll の前に確認します。
`Pending` の後にも、もう一度確認します。許された最後の poll が `Pending` を返したら、`Budget` はその同じ
poll の中であきらめなければなりません。いったん `Pending` を返して次の poll であきらめると、呼び出し側の
poll が 1 回むだになり、内側の Future も必要以上に長く残ります。

`Budget` は `cx` をそのまま渡します。内側の Future は、自分が poll されたときの `Context` の waker を起こし、
それによって実行器は `Budget` が先に進めることを知ります。`Waker::noop()` などの自前の waker で内側の
Future を poll すると、その通知が失われます。`Budget` が `Pending` を返すのは内側の Future が `Pending` を
返した直後だけなので、`Budget` が自分で何かを起こす必要はありません。

### 別解

`Option<F>` を入れた box を固定する方法もあります。`Option::as_pin_mut` は `Pin<&mut Option<F>>` を
`Option<Pin<&mut F>>` に変え、`Pin::set` は固定された値をその場で置き換えて、古い値を破棄します。
`set(None)` は、内側の Future を一度も動かさずに破棄します。直接持った `Option<F>` をピンの射影で扱う
場合も同じ 2 つの呼び出しを使いますが、box のおかげで安全に書けます。

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Budget<F> {
    /// Pinned on the heap. `None` once the inner future has been dropped.
    inner: Pin<Box<Option<F>>>,
    left: u32,
}

pub fn with_budget<F: Future>(fut: F, polls: u32) -> Budget<F> {
    Budget { inner: Box::pin(Some(fut)), left: polls }
}

impl<F: Future> Future for Budget<F> {
    type Output = Option<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        // `as_pin_mut`: `Pin<&mut Option<F>>` -> `Option<Pin<&mut F>>`.
        let output = match this.inner.as_mut().as_pin_mut() {
            Some(fut) if this.left > 0 => {
                this.left -= 1;
                match fut.poll(cx) {
                    Poll::Ready(output) => Some(output),
                    Poll::Pending if this.left > 0 => return Poll::Pending,
                    Poll::Pending => None,
                }
            }
            // A budget of 0: give up without polling.
            _ => None,
        };
        // Drops the inner future in place, without moving it.
        this.inner.set(None);
        Poll::Ready(output)
    }
}
```

poll の間だけ box を `Option` から取り出し、まだ必要なときだけ戻す方法もあります。取り出しても動くのは
ポインタだけなので、`Unpin` でない Future でも問題ありません。box を戻さなかったときは、`poll` の終わりで
box がスコープを抜けるので、内側の Future は `poll` から戻る前に破棄されます。この版では、フィールドへの
書き込みに `Pin` の `DerefMut` を使っています（そのため `mut self` にしています）。これも `get_mut` と同じく
`Budget<F>: Unpin` が必要です。模範解答の `poll` と置き換えて使います。

```rust
fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    // Moving the box moves only the pointer; the future stays where it is.
    let mut fut = self.inner.take().expect("a Budget is not polled after it is ready");
    if self.left == 0 {
        return Poll::Ready(None);
    }
    self.left -= 1;
    match fut.as_mut().poll(cx) {
        Poll::Ready(output) => Poll::Ready(Some(output)),
        Poll::Pending if self.left > 0 => {
            // Still needed: put it back.
            self.inner = Some(fut);
            Poll::Pending
        }
        Poll::Pending => Poll::Ready(None),
    }
    // Unless it was put back, `fut` is dropped here, before `poll` returns.
}
```
