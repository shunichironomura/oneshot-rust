複数の Future を 1 つの Future にまとめる `join` と `join_all` を書いてください。まとめた Future は、
現在のスレッドの上でそれらを順番に poll して、並行に実行します。非同期ランタイムはありません
（使えるのは `std` だけです）。

```rust
pub fn join<A: Future, B: Future>(a: A, b: B) -> impl Future<Output = (A::Output, B::Output)>;
pub fn join_all<F: Future>(futs: Vec<F>) -> impl Future<Output = Vec<F::Output>>;
```

以下では、`join` や `join_all` が返す Future を「結合した Future」、`a` と `b`（または `futs` の要素）を
その「入力」と呼びます。

- `join` や `join_all` を呼んだ時点では何も poll しません。入力が poll されるのは、結合した Future が
  poll されたときだけです。
- 結合した Future は、poll されるたびに、まだ完了していない入力をそれぞれ 1 回ずつ順に poll します。
  順序は `a`、`b` の順、`futs` は添字の順です。ある入力が `Poll::Pending` を返しても、その後の入力は
  poll します。
- `Poll::Ready` を返した入力は完了しています。完了した入力は二度と poll しません。
- 結合した Future は、最後の入力が完了したのと同じ poll で `Poll::Ready` を返し、それより前の poll
  ではすべて `Poll::Pending` を返します。出力は入力の順に並べます。`join` なら `(a の出力, b の出力)`、
  `join_all` なら `futs` の各要素の出力を並べた `Vec` です。`futs` が空のときは、最初の poll で空の
  `Vec` を返して完了します。
- 入力は、結合した Future がそのとき poll されている `Context` で poll します。結合した Future 自身は
  waker を起こしません。起こすのは入力の役目です。
- 入力はどんな Future でもかまいません。ローカル変数を借用していて、`Unpin` でも `Send` でもない
  `async` ブロックも渡されます。出力の型も任意で、`Clone`、`Copy`、`Default`、`Unpin` のどれも
  実装していないことがあります。

テストハーネスは、独自の実行器を使って、結合した Future を現在のスレッドで実行します。実際の実行器と
同じく、Future を再び poll するのは waker が起こされた後だけです。waker が起こされないまま
`Poll::Pending` を返した Future は二度と poll されないことになるので、ハーネスはそれを失敗として
報告します。`Poll::Ready` を返した Future を再び poll することはありません。

### 例

例の中の `steps(["a0", "a1"])` はハーネスのタスクを表します。渡されたステップを 1 つずつログに記録する
`async` ブロックで、ステップとステップの間で 1 回譲ります（waker を起こしてから `Poll::Pending` を
返します）。つまり、最後のステップを記録した poll で完了し（ステップがなければ最初の poll で完了し）、
ステップの数を出力します。

- `join(steps(["a0", "a1"]), steps(["b0", "b1", "b2"]))` は `a0`、`b0`、`a1`、`b1`、`b2` の順に
  ログを記録し、3 回目の poll で `(2, 3)` を返して完了します。1 つ目の入力は 2 回目の poll で完了する
  ので、3 回目の poll では 2 つ目の入力だけを poll します。
- `join_all(vec![steps(["x0", "x1", "x2"]), steps(["y0"]), steps(["z0", "z1"])])` は `x0`、`y0`、
  `z0`、`x1`、`z1`、`x2` の順にログを記録し、3 回目の poll で `[3, 1, 2]` を返して完了します。
