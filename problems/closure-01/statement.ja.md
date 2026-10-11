新しい関数を作って返す関数を 2 つ実装してください。

```rust
pub fn make_counter(step: u32) -> impl FnMut() -> u32
pub fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C
```

- `make_counter(step)` はカウンターを返します。呼ぶたびに `step` の次の倍数（`step`、`2 * step`、`3 * step`、…）を返します。
  カウンターはそれぞれ別々に数えます。
- `compose(f, g)` は、まず `f` を適用し、その結果に `g` を適用する関数を返します。

### 例

```rust
let mut c = make_counter(3);
// c() は 3、6、9 の順に返す

let h = compose(|x: i32| x + 1, |x: i32| x * 10);
// h(4) は 50 を返す
```
