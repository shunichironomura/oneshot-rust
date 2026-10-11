future を `Budget` で包む関数 `with_budget` を書いてください。`Budget` は包んだ future を実行する future ですが、
決められた回数だけ poll しても終わらなければ、あきらめて終わります。非同期ランタイムはありません
（使えるのは `std` だけです）。次のひな形はエディタに入っています。`Budget` のフィールドは自分で決めてください。

```rust
pub struct Budget<F> { /* private */ }

pub fn with_budget<F: Future>(fut: F, polls: u32) -> Budget<F>;

impl<F: Future> Future for Budget<F> {
    type Output = Option<F::Output>;
    /* poll */
}
```

以下では、`fut` を*内側の future* と呼びます。

- `with_budget` を呼んだ時点では、何も poll せず、何も破棄しません。
- `Budget` は poll されるたびに、自分が poll されたときの `Context` をそのまま渡して、内側の future を
  1 回 poll します。その結果に応じて、次のようにします。
  - 内側の future が `Poll::Ready(v)` を返したら、`Budget` は `Poll::Ready(Some(v))` を返します。
  - `Poll::Pending` を返したら、`Budget` も `Poll::Pending` を返します。ただし、それが内側の future の
    `polls` 回目の poll だったときは、予算を使い切ったので、`Budget` はすぐにあきらめます。つまり、
    その同じ poll の中で `Poll::Ready(None)` を返します。
- `polls` が `0` のときは、`Budget` の最初の poll で、内側の future を poll せずに `Poll::Ready(None)` を
  返します。
- 内側の future は、不要になったらすぐに破棄します。`Budget` が `Poll::Ready` を返すときは、その poll の中で
  （戻る前に）内側の future をすでに破棄しています。`Budget` 自体が破棄されるまで待つことはしません。
  準備完了になる前に破棄された `Budget` は、内側の future も一緒に破棄します。
- `Budget` が自分で waker を起こすことはありません。起こすのは内側の future の役目です。
- 内側の future はどんな future でもかまいません。ローカル変数を借用していて、`Unpin` でも `Send` でもない
  `async` ブロックのこともあります。出力の型も何でもかまいません。

テストハーネスは future を現在のスレッドで poll します。`Poll::Ready` を返した `Budget` を再び poll することは
ありません。

### 例

例の中の `ticks(n, out)` は、ハーネスが用意する `Unpin` でない future を表します。最初の `n` 回の poll では、
そのたびに waker を起こしてから `Poll::Pending` を返し、その次の poll で `Poll::Ready(out)` を返します。

- `with_budget(ticks(2, "done"), 3)` は、最初の 2 回の poll で `Pending` を、3 回目で `Ready(Some("done"))` を
  返します。3 回目の poll から戻った時点で、`ticks` の future は 3 回 poll されて、すでに破棄されています
  （`Budget` 自体はまだ残っています）。
- `with_budget(ticks(5, "late"), 3)` は、最初の 2 回の poll で `Pending` を、3 回目で `Ready(None)` を返します。
  `ticks` の future が 3 回の poll のどれでも `Pending` を返し、予算を使い切ったからです。3 回目の poll から
  戻った時点で、`ticks` の future はすでに破棄されています。
