`W` は 2 つのウィジェットトレイトを両方実装していて、どちらにも `get(&self)` があるので、メソッド呼び出し構文では
選べません（E0034）。`let age: u8` のような型注釈は役に立ちません。Rust はメソッドを名前とレシーバで選び、
呼び出し側が期待する返り値の型では選ばないからです。パスにトレイト名を書いて `UsernameWidget::get(w)` とします。
引数 `w: &W` から、`Self` が `W` だとわかります。

ここではコンパイラの提案をそのまま使わないようにしましょう。`UsernameWidget::get(&w)` が提案されます。
パスにトレイト名しか書かないと、`Self` は引数の型から推論されます。パスでの呼び出しは、メソッド呼び出し構文の
ような自動参照・自動参照外しをしません。`&w` は `&&W` なので `Self` が `&W` になり、`&W` はどちらのトレイトも
実装していません。`w` をそのまま渡します（`<W as UsernameWidget>::get(&w)` ならコンパイルが通ります。
`Self` が決まっていれば、`&&W` は `&W` に型強制されるからです）。

### 別解

完全修飾構文 `<Type as Trait>::f(...)` を使えば、型も明示できます。

```rust
pub fn summary<W: UsernameWidget + AgeWidget>(w: &W) -> String {
    format!("{} ({})", <W as UsernameWidget>::get(w), <W as AgeWidget>::get(w))
}
```

メソッド呼び出し構文が曖昧になるのは、両方のトレイトが当てはまる場所だけです。境界が 1 つだけの型引数の
後ろでは、`x.get()` からはそのトレイトしか見えません。そのため、小さなヘルパーを挟むと呼び出しの曖昧さが
なくなります。

```rust
fn username_of<U: UsernameWidget>(u: &U) -> String {
    u.get()
}

fn age_of<A: AgeWidget>(a: &A) -> u8 {
    a.get()
}

pub fn summary<W: UsernameWidget + AgeWidget>(w: &W) -> String {
    format!("{} ({})", username_of(w), age_of(w))
}
```
