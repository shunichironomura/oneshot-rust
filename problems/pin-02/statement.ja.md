Future を `Budget` で包む関数 `with_budget` を書いてください。`Budget` は包んだ Future を実行する
Future ですが、決められた回数だけポーリングしても終わらなければ、あきらめて終わります。非同期ランタイムは
ありません（使えるのは `std` だけです）。構造体 `Budget` は 2 つのフィールドとともにエディタに入っています。

```rust
pub struct Budget<F> {
    inner: Option<F>,
    left: u32,
}

pub fn with_budget<F: Future>(fut: F, polls: u32) -> Budget<F>;

impl<F: Future> Future for Budget<F> {
    type Output = Option<F::Output>;
    /* poll */
}
```

以下では、`fut` を*内側の Future*、`polls` を*予算*と呼びます。

- 非同期ライブラリのコンビネータと同じように、`Budget` は内側の Future を自分の中に持ち、Future を
  包むときにメモリを確保しません。フィールドは変えないでください。内側の Future をヒープに移しても
  いけません。`size_of::<Budget<F>>()` は `size_of::<F>()` 以上です。`Budget<F>` は、`F` が
  `Unpin` のときに限り `Unpin` です。
- `with_budget` を呼んだ時点では、何もポーリングせず、何も破棄しません。
- 予算が `0` のときは、`Budget` の最初のポーリングで、内側の Future をポーリングせずに
  `Poll::Ready(None)` を返します。
- それ以外のときは、`Budget` はポーリングされるたびに、自分がポーリングされたときの `Context` を
  そのまま渡して、内側の Future を 1 回ポーリングします。その結果に応じて、次のようにします。
  - 内側の Future が `Poll::Ready(v)` を返したら、`Budget` は `Poll::Ready(Some(v))` を返します。
  - `Poll::Pending` を返したら、`Budget` も `Poll::Pending` を返します。ただし、それが内側の Future の
    `polls` 回目のポーリングだったときは、予算を使い切ったので、`Budget` はすぐにあきらめます。
    つまり、その同じポーリングの中で `Poll::Ready(None)` を返します。
- 内側の Future は、不要になったらすぐに破棄します。`Budget` が `Poll::Ready` を返すときは、その
  ポーリングの中で（戻る前に）内側の Future をすでに破棄しています。`Budget` 自体が破棄されるまで
  待つことはしません。完了する前に破棄された `Budget` は、内側の Future も一緒に破棄します。
- 内側の Future は、一度ポーリングされたら、破棄されるまでメモリ上の同じ場所にとどまります。
  `Pin` がそれを要求するからです。
- `Budget` が自分でウェイカーを起こすことはありません。起こすのは内側の Future の役目です。
- 内側の Future はどんな Future でもかまいません。ローカル変数を（`.await` をまたいで自分自身の
  ローカル変数も）借用していて、`Unpin` でも `Send` でもない `async` ブロックのこともあります。
  出力の型も何でもかまいません。
- `unsafe` を使ってかまいません。未定義動作やメモリリークは Miri が検出し、不合格になります。

テストハーネスは Future を現在のスレッドでポーリングします。いくつかのテストでは、実際の
エグゼキュータと同じように、ウェイカーが起こされた後でだけ Future を再びポーリングします。
`Poll::Ready` を返した `Budget` を再びポーリングすることはありません。

### 例

例の中の `ticks(n, out)` は、ハーネスが用意する `Unpin` でない Future を表します。最初の `n` 回の
ポーリングでは、そのたびにウェイカーを起こしてから `Poll::Pending` を返し、その次のポーリングで
`Poll::Ready(out)` を返します。

- `with_budget(ticks(2, "done"), 3)` は、最初の 2 回のポーリングで `Pending` を、3 回目で
  `Ready(Some("done"))` を返します。3 回目のポーリングから戻った時点で、`ticks` の Future は 3 回
  ポーリングされて、すでに破棄されています（`Budget` 自体はまだ残っています）。
- `with_budget(ticks(5, "late"), 3)` は、最初の 2 回のポーリングで `Pending` を、3 回目で
  `Ready(None)` を返します。`ticks` の Future が 3 回のポーリングのどれでも `Pending` を返し、
  予算を使い切ったからです。3 回目のポーリングから戻った時点で、`ticks` の Future はすでに
  破棄されています。
