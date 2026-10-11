`async` を使わずに手で Future を書き、Future を完了まで実行する小さなエグゼキュータを作ってください。
下の骨組みはエディタに入っています。`Countdown` と `Delay` のフィールドは自由に決めてください。

```rust
pub struct Countdown { /* private */ }
pub fn countdown(n: u32) -> Countdown;
impl Future for Countdown { type Output = u32; /* poll */ }

pub struct Delay { /* private */ }
impl Delay {
    pub fn new(polls: u32) -> Self;
    pub fn extra(self, k: u32) -> Self;
}
impl IntoFuture for Delay { type Output = u32; type IntoFuture = Countdown; /* into_future */ }

pub fn block_on<F: IntoFuture>(f: F) -> F::Output;
pub fn poll_count<F: Future>(fut: F) -> (F::Output, usize);
```

- `countdown(n)` は Future を返します。この Future は、最初の `n` 回のポーリングでは
  `Poll::Pending` を返し、その次のポーリングで `Poll::Ready(n)` を返します。
  完了した後にもう一度ポーリングされることはありません。
- `Countdown` は `Pending` を返すときは毎回、返す前に、その `poll` 呼び出しに渡された `Context` の
  ウェイカーの `wake_by_ref()` をちょうど 1 回呼びます。`Ready` を返すときは何も起こしません。
- `Delay` はポーリング回数のビルダーです。`Delay::new(polls)` の合計は `polls` から始まり、
  `extra(k)` を呼ぶたびに合計に `k` が足されます。`extra` は何回でも（0 回でも）呼べます。
  `into_future()`（`.await` が呼ぶメソッド）は、合計を渡した `countdown` と同じように動く
  `Countdown` を返します。たとえば `Delay::new(2).extra(1)` を await するのは、`countdown(3)` を
  await するのと同じです。合計は必ず `u32` に収まります。
- `block_on(f)` は `f` を Future に変換し、何もしないウェイカーを使って、完了するまで
  現在のスレッドで何度もポーリングします。完了したら出力を返します。
- `poll_count(fut)` は同じように `fut` を実行し、出力と、`fut` の `poll` を呼んだ回数の組を返します。
  回数には `Ready` を返した最後の呼び出しも含めます。
- `block_on` と `poll_count` は、`async` ブロックを含め、どんな Future も受け付けなければなりません。

### 例

- `poll_count(countdown(3))` は `(3, 4)` です（`Pending` が 3 回、その後 `Ready(3)`）。
- `block_on(Delay::new(2).extra(1))` は `3` です。
- `block_on(async { countdown(2).await + Delay::new(1).await })` は `3` です。
- `poll_count(async { 5 })` は `(5, 1)` です。
