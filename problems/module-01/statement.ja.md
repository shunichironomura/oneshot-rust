次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

小さなお店のコードがモジュールに分かれています。`shop` には商品、`shop::cart` には買い物かご、
`shop::pricing` には価格の計算が入っています。あなたのコードはモジュール `crate::user` です。
テストハーネスは同じクレートの別のモジュールで、`crate::user` の外にあります。ハーネスは次のパスを
そのまま使います。

```rust
crate::user::shop::TAX_RATE // u32 の定数、10
crate::user::shop::Item::new(name: &str, price: u32) -> Item
crate::user::shop::Item::name(&self) -> &str
crate::user::shop::Item::price(&self) -> u32
crate::user::shop::cart::Cart::new() -> Cart
crate::user::shop::cart::Cart::add(&mut self, item: Item)
crate::user::shop::cart::Cart::total(&self) -> u32
crate::user::shop::cart::Cart::len(&self) -> usize
crate::user::sample_cart() -> Cart
crate::user::checkout(cart: &Cart, percent: u32) -> u32
```

- 価格の単位はセントです。割り算はすべて整数の割り算で、切り捨てになります。
- `Item::new` は商品を作ります。`name` と `price` は、作ったときの値を返します。
- `Cart::new` は空のかごを作り、`add` はかごに商品を入れます。`total` はかごの中の全商品の価格の合計
  （空のかごなら `0`）、`len` は商品の数です。
- `sample_cart` は、ペン（`120`）と本（`1500`）が入った新しいかごを返します。
- `checkout(cart, percent)` は支払う金額を返します。まずかごの合計 `t` から割引額 `t * percent / 100`
  を引き、次に割引後の価格 `d` に税額 `d * TAX_RATE / 100` を足した値です。`percent` は `0` 以上
  `100` 以下です。
- `Item` と `Cart` のフィールドは非公開のままにし、計算の内容は変えないでください。
  それ以外（可視性、パス、`use` 宣言、関数の処理の書き方など）は変えても構いません。

### 例

`sample_cart().len()` は `2`、`sample_cart().total()` は `1620` です。
`checkout(&sample_cart(), 10)` は `1603` です。割引額は `162` で残りは `1458`、それにかかる税額は
`145` です。
