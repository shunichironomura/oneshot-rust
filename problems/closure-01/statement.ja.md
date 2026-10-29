カウンターを作り、クロージャとして返す関数を実装してください。

```rust
pub fn make_counter(step: u32) -> impl FnMut() -> u32
```

- `make_counter(step)` はカウンターを返します。呼ぶたびに `step` の次の倍数（`step`、`2 * step`、`3 * step`、…）を返します。
- カウンターはそれぞれ別々に数えます。

### 例

```rust
let mut c = make_counter(3);
// c() は 3、6、9 の順に返す
```
