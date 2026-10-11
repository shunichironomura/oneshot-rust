`poll` が受け取るのは `self: Pin<&mut Self>` ですが、フィールド（予算や、破棄するときの内側の Future）を
変更するには可変のアクセスが必要です。`Pin` が `&mut Self` を渡してくれるのは `Self: Unpin` のときだけです
（`get_mut` や、`mut self` にしたときの `DerefMut` で）。`Unpin` は自動トレイトです。`Budget<F>` は
`Option<F>` を直接持っているので、`Unpin` になるのは `F` が `Unpin` のときだけで、ハーネスの `async`
ブロックや `ticks` の Future は `Unpin` ではありません。そのため、`self.left -= 1` は「cannot assign to data
in dereference of `Pin<&mut Budget<F>>`」（E0594）で、`self.get_mut()` や内側の Future への
`Pin::new(&mut ...)` は「`F` cannot be unpinned」（E0277）で拒否されます。コンパイラの提案どおり
`F: Unpin` を付けても、エラーがハーネス側に移るだけです。内側の Future を `Box::pin` でヒープ上に
ピン留めすれば `Budget<F>` は `Unpin` になりますが、問題文がそれを禁じています。非同期ライブラリの
コンビネータと同じく、`Budget` はメモリを確保しません。

残る方法は*ピンの射影*（pin projection）です。`Pin<&mut Budget<F>>` を、フィールド `inner` については
`Pin<&mut Option<F>>` に、`left` については普通の `&mut u32` に変えます。自分で定義した型の射影を安全に
書く方法は Rust にはない（`pin-project` クレートが生成してくれます）ので、`unsafe` で書きます。
`self.get_unchecked_mut()` で `&mut Budget<F>` を得て、`Pin::new_unchecked(&mut this.inner)` で
フィールドをピン留めし直します。`unsafe` の部分は約束です。フィールドごとに決めて、それを守ります。

- `inner` は*構造的にピン留め*されます。一度ポーリングしたら、二度と動かしません。`Option::take`、
  `mem::replace`、`mem::swap` で動かしてはいけませんし、破棄するために動かすのもいけません。
  その場で破棄しなければなりません。`Budget` に `Unpin` を実装してはいけません（実装すると、安全な
  コードが `Pin` から `&mut Budget<F>` を取り出して、内側の Future を動かせてしまいます）。
  フィールドを動かすコードを含む `Drop` を実装してもいけません。
- `left` はピン留めされません。`&mut u32` を渡しても、内側の Future が動くことはありません。

`Pin<&mut Option<F>>` があれば、`as_pin_mut()` でポーリングに使う `Option<Pin<&mut F>>` が得られ、
`set(None)` で内側の Future をその場で破棄して `None` を入れられます。`as_mut()` でピンを借用し直すので、
その後も `set` に使えます。

落とし穴は `take()` です。フィールドが `Option` だと、Future を取り出してポーリングし、また戻す、
あるいは破棄するために取り出す、というコードを書きたくなります。動かした Future に
`unsafe { Pin::new_unchecked(&mut fut) }` を使えばコンパイルは通りますが、約束を破っています。
`.await` をまたいで自分のローカル変数を借用する `async` ブロックは、自分自身の中を指すポインタを
持っていて、動かした後もそのポインタは元の場所を指したままです（テストの 1 つでは、ブロック自身の
ローカル配列を `for x in &items` でたどるループの中で `.await` しています）。すると次のポーリングで
ダングリングポインタをたどることになり、Miri が未定義動作を報告します。ハーネスの `ticks` の
Future も、最初にポーリングされた場所にとどまっているかを確かめています。

内側の Future を早めに破棄することが、Rust での Future のキャンセルです。`.await` の途中で破棄された
`async` ブロックは二度と再開されず、その時点で持っている値が破棄されます。`Ready` を返した後でも、
破棄することには意味があります。完了した `async` ブロックはローカル変数やキャプチャをすでに破棄して
いますが、`ticks` のような手書きの Future は、破棄されるまで自分の持ち物を持ち続けるからです。

回数は慎重に数えましょう。予算 `0` は「一度もポーリングしない」という意味なので、ポーリングの前に
確認します。`Pending` の後にも、もう一度確認します。許された最後のポーリングが `Pending` を返したら、
`Budget` はその同じポーリングの中であきらめます。`cx` はそのまま渡します。内側の Future は、自分が
ポーリングされたときの `Context` のウェイカーを起こし、それによってエグゼキュータは `Budget` が先に
進めることを知ります。`Budget` が `Pending` を返すのは内側の Future が `Pending` を返した直後だけなので、
`Budget` が自分で何かを起こす必要はありません。

### 別解

射影を、ピン留めされた `Budget` をフィールドに分けるメソッドとして一度だけ書く方法です。`pin-project` が
生成するのもこの形で、`unsafe` なコードを 1 か所にまとめられます。

```rust
impl<F> Budget<F> {
    /// The pin projection: `inner` is structurally pinned, `left` is not.
    fn project(self: Pin<&mut Self>) -> (Pin<&mut Option<F>>, &mut u32) {
        // SAFETY: `inner` is only ever used through the `Pin` returned here, so it is never
        // moved out of a pinned `Budget`. `left` is not pinned.
        let this = unsafe { self.get_unchecked_mut() };
        (unsafe { Pin::new_unchecked(&mut this.inner) }, &mut this.left)
    }
}

impl<F: Future> Future for Budget<F> {
    type Output = Option<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let (mut inner, left) = self.project();
        let mut output = None;
        if *left > 0
            && let Some(fut) = inner.as_mut().as_pin_mut()
        {
            *left -= 1;
            match fut.poll(cx) {
                Poll::Ready(value) => output = Some(value),
                Poll::Pending if *left > 0 => return Poll::Pending,
                Poll::Pending => {}
            }
        }
        inner.set(None);
        Poll::Ready(output)
    }
}
```

`Pin::set` はただの代入で、代入は新しい値を書き込む前に古い値をその場で破棄します。そのため、
`&mut Budget<F>` に対して `this.inner = None` と書いて内側の Future を破棄し、ピン留めは `poll` の
呼び出しのときだけにすることもできます。模範解答の `poll` と置き換えて使います。

```rust
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    // SAFETY: nothing below moves the inner future out of the pinned `Budget`.
    let this = unsafe { self.get_unchecked_mut() };
    let mut output = None;
    if this.left > 0
        && let Some(fut) = &mut this.inner
    {
        this.left -= 1;
        // SAFETY: `fut` lives inside the pinned `Budget` and is never moved.
        match unsafe { Pin::new_unchecked(fut) }.poll(cx) {
            Poll::Ready(value) => output = Some(value),
            Poll::Pending if this.left > 0 => return Poll::Pending,
            Poll::Pending => {}
        }
    }
    // Assigning drops the old value in place, so the inner future is not moved.
    this.inner = None;
    Poll::Ready(output)
}
```
