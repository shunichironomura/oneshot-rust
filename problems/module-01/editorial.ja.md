Rust の可視性はモジュール単位で決まります。`pub` のない要素は、それを定義したモジュールと、
その子孫のモジュールからだけ見えます。構造体に `pub` を付けても公開されるのは型そのものだけで、
フィールド、メソッド、関連関数はそれぞれ別に可視性を持ちます。

テンプレートには 4 つの問題があります。

- `pricing` の中で `TAX_RATE` が見つかりません。`mod` ブロックは新しい名前空間を作るので、
  親モジュールの名前はその中では使えません。`super::TAX_RATE` と書くか、`use super::TAX_RATE;` とします。
  定数が `shop` の非公開要素であることは問題になりません。`pricing` は `shop` の中にあるからです。
- `pricing` は `shop` の非公開モジュールなので、`shop` の外にある `checkout` からは、
  中の関数が `pub` でも使えません。パスの途中のモジュールもすべて見えている必要があります。
  `pub(super) mod pricing` とすると、`shop` の親（`checkout` がある場所）までに限って公開できます。
  ただの `pub` でも構いません。
- `Item::new` と `Item::price` は非公開です。また、`crate::user` の外にあるハーネスは `TAX_RATE` も読みます。
  これらには `pub`（または `pub(crate)`）が必要です。`pub(super)` では `crate::user` までしか届かないので足りません。
- `sample_cart` は構造体リテラルで `Cart` を作っていますが、`items` は `shop::cart` の非公開フィールドです。
  構造体リテラルはすべてのフィールドを書くので、書いた場所からすべてのフィールドが見えている必要があります。
  フィールドは非公開のままにするので、公開されている `Cart::new` と `add` を使ってかごを作ります。
  コンパイラはフィールドの可視性を後の段階で調べるので、このエラーはほかのエラーを直した後で初めて報告されます。

怪しく見えても問題ない行もあります。`Cart::total` は `Item` の非公開フィールド `item.price` を読んでいます。
このフィールドは `shop` で非公開になっていて、`shop::cart` は `shop` の中にあるので見えます。
`price` を `pub` フィールドにしてもコンパイルは通りますが、誰からでも見えるようになってしまいます。
これは問題文で禁止されています。

### 別解

`pricing` は非公開のままにもできます。`pricing` が見える `shop` の中に `checkout` を移し、
再エクスポートします。`pub use` によって、ハーネスが使うパス `crate::user::checkout` で使えるようになります。

```rust
pub mod shop {
    // TAX_RATE、Item、cart は模範解答と同じ

    mod pricing {
        pub fn discounted(total: u32, percent: u32) -> u32 {
            total - total * percent / 100
        }

        pub fn with_tax(total: u32) -> u32 {
            total + total * super::TAX_RATE / 100
        }
    }

    pub fn checkout(cart: &cart::Cart, percent: u32) -> u32 {
        pricing::with_tax(pricing::discounted(cart.total(), percent))
    }
}

pub use shop::checkout;

// sample_cart は模範解答と同じ
```

`use` 宣言を使うと、`crate::user` の中のパスを短くできます。ハーネスは同じクレートにあるので、
`TAX_RATE`、`Item::new`、`Item::price`、`pricing` には `pub(crate)` を使っても構いません
（`pub(crate) mod pricing`）。

```rust
// shop は模範解答と同じ

use shop::cart::Cart;
use shop::pricing::{discounted, with_tax};

pub fn sample_cart() -> Cart {
    let mut cart = Cart::new();
    for (name, price) in [("pen", 120), ("book", 1500)] {
        cart.add(shop::Item::new(name, price));
    }
    cart
}

pub fn checkout(cart: &Cart, percent: u32) -> u32 {
    with_tax(discounted(cart.total(), percent))
}
```
