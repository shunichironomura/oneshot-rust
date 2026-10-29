`UsernameWidget::kind()` はトレイト名を書いていますが、`kind` には `self` 引数がないので、どの型の実装を
使うかを決める手がかりがありません（E0790）。メソッドならレシーバの型がその手がかりになりますが、関連関数では
型を自分で書く必要があります。完全修飾構文で `<W as UsernameWidget>::kind()` とします。`W::kind()` も
うまくいきません。`W` にはどちらのトレイトの `kind` もあるからです（E0034）。

ここではコンパイラの提案をそのまま使わないようにしましょう。`<Form as UsernameWidget>::kind()` や、そこを
ハーネスの型にしたものが提案されますが、`kinds` は `W` が何であっても `W` に問い合わせる必要があります。
ハーネスの型は独自の `kind` を持っています。

### 別解

トレイト 1 つだけを境界に持つジェネリックなヘルパーを挟むと、パスの曖昧さがなくなります。境界が 1 つだけの
型引数では、`U::kind()` はそのトレイトの `kind` しか指せません。

```rust
fn username_kind<U: UsernameWidget>() -> &'static str {
    U::kind()
}

fn age_kind<A: AgeWidget>() -> &'static str {
    A::kind()
}

pub fn kinds<W: UsernameWidget + AgeWidget>() -> (&'static str, &'static str) {
    (username_kind::<W>(), age_kind::<W>())
}
```
