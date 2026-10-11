4 つの関数のうち 3 つは曖昧な呼び出しをしていて、残りの 1 つはコンパイルは通るのに違うメソッドを呼んでいます。

- `summary` の `w.get()`：`W` は 2 つのウィジェットトレイトを両方実装していて、どちらにも `get(&self)` が
  あるので、メソッド呼び出し構文では選べません（E0034）。`let age: u8` のような型注釈は役に立ちません。
  Rust はメソッドを名前とレシーバで選び、呼び出し側が期待する返り値の型では選ばないからです。
  パスにトレイト名を書いて `UsernameWidget::get(w)` とします。引数 `w: &W` から、`Self` が `W` だと
  わかります。`next_age` の `form.get()` も、具体的な型で起きている同じ問題です（`AgeWidget::get(form)`）。
- `kinds` の `UsernameWidget::kind()`：`kind` には `self` 引数がないので、どの型の実装を使うかを決める
  手がかりがありません（E0790）。完全修飾構文で型も書いて `<W as UsernameWidget>::kind()` とします。
  `W::kind()` もうまくいきません。`W` にはどちらのトレイトの `kind` もあるからです（これも E0034）。
- `tag` の `form.label()` はコンパイルが通りますが、`"form of rustacean"` を返します。名前もレシーバも
  同じ（ここではどちらも `&self`）固有メソッドとトレイトのメソッドがあると、メソッド呼び出し構文は
  固有メソッドを選びます。`Labeled::label(form)` と書けばトレイトのメソッドを呼べます。

ここではコンパイラの提案をそのまま使わないようにしましょう。`kind` については
`<Form as UsernameWidget>::kind()` や、そこをハーネスの型にしたものが提案されますが、`kinds` は `W` が
何であっても `W` に問い合わせる必要があります。ハーネスの型は独自の `kind` を持っています。`get` については
`UsernameWidget::get(&w)` が提案されます。パスにトレイト名しか書かないと、`Self` は引数の型から推論されます。
`&w` は `&&W` なので `Self` が `&W` になり、`&W` はどちらのトレイトも実装していません。`w` をそのまま渡します
（`<W as UsernameWidget>::get(&w)` ならコンパイルが通ります。`Self` が決まっていれば、`&&W` は `&W` に
型強制されるからです）。

`saturating_add` は最初から正しく書けています。`age + 1` だと、年齢が `255` のときにオーバーフローで
パニックします。

### 別解

完全修飾構文 `<Type as Trait>::f(...)` はメソッドにも使えて、どこでも型を明示できます。

```rust
pub fn summary<W: UsernameWidget + AgeWidget>(w: &W) -> String {
    format!("{} ({})", <W as UsernameWidget>::get(w), <W as AgeWidget>::get(w))
}

pub fn next_age(form: &Form) -> u8 {
    <Form as AgeWidget>::get(form).saturating_add(1)
}

pub fn tag(form: &Form) -> String {
    <Form as Labeled>::label(form)
}
```

メソッド呼び出し構文が曖昧になるのは、両方のトレイトが当てはまる場所だけです。境界が 1 つだけの型引数の
後ろでは、`x.get()` や `x.label()` からはそのトレイトしか見えません（型引数には固有メソッドがありません）。
そのため、小さなヘルパーを挟むと呼び出しの曖昧さがなくなります。

```rust
fn username_of<U: UsernameWidget>(u: &U) -> String {
    u.get()
}

fn age_of<A: AgeWidget>(a: &A) -> u8 {
    a.get()
}

fn label_of<L: Labeled>(l: &L) -> String {
    l.label()
}

pub fn summary<W: UsernameWidget + AgeWidget>(w: &W) -> String {
    format!("{} ({})", username_of(w), age_of(w))
}

pub fn next_age(form: &Form) -> u8 {
    age_of(form).saturating_add(1)
}

pub fn tag(form: &Form) -> String {
    label_of(form)
}
```
