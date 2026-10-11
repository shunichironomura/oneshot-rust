テンプレートの `adopt` は、呼ぶたびに**循環参照**を作ります。親の `children` が子への `Rc` を持ち、
子の `parent` が逆向きに親への `Rc` を持っているからです。`Rc` が値を解放するのは強参照カウントが 0 になったときだけですが、
循環の中ではお互いが相手のカウントを 0 より大きく保ちます。呼び出し側がハンドルを手放すと、もう誰もこれらのノードに
たどり着けないのに、解放もされません。そのため、プログラムの終わりに Miri がリークとして報告します。
カウントがずれる（`Rc::strong_count(&root)` が 2 になる）のも、`drop(root)` の後もパスに `root` が残るのも、
同じ循環が原因です。

直すには、各リンクの片方向を所有しない参照にします。子を所有するのは親なので、`children` は `Vec<Rc<Node>>` のままにし、
親へ戻るポインタを `Weak<Node>` にします。

- `Weak::new()` は何も指していない `Weak` です。親のいないノードで `None` の代わりに使います。
- `Rc::downgrade(parent)` は親への `Weak` を作ります。増えるのは弱参照カウントで、強参照カウントは増えないので、
  子が親を生かすことはありません。`Rc::strong_count` や `Rc::ptr_eq` と同じく、メソッドではなく関連関数です
  （`Node` のメソッドを隠さないようにするため）。`parent.downgrade()` とは書けません。
- `upgrade()` は `Weak` を `Option<Rc<Node>>` に戻します。親が生きていれば `Some`、解放された後は `None` です。
  これはちょうど `parent_name` と `path` に必要なものです。結果がまた `Option` になるので、テンプレートの `map` と
  `while let` はそのまま使えます。親に対する `.as_ref()` と `.clone()` を `.upgrade()` に変えるだけです。

`adopt` では、`RefCell::replace` で新しい親を書き込むと同時に、古い `Weak` を受け取れます。古い親はすでに
解放されているかもしれません。そのときは `upgrade()` が `None` を返すので、子をどこからも取り除く必要はありません。
`Rc::ptr_eq` はノードを同一性で比べるので、同じ名前の兄弟は残ります。

1 つの変数で親をたどろうとすると、コンパイルが通りません。

```rust
let mut cur = Rc::clone(n);
while let Some(p) = cur.parent.borrow().upgrade() {
    names.push(p.name.clone());
    cur = p; // error[E0506]: cannot assign to `cur` because it is borrowed
}
```

`borrow()` が返す `Ref` は `while let` の条件式の一時値で、ループ本体の終わりまで生きています。
そのため、`cur` に代入する時点では `cur` がまだ借用されています。模範解答では、次の親を読む処理を独立した文にしています。
文の一時値はその `;` で破棄されます。

両方向を `Weak` にしてもリークはなくなりますが、子を所有するものがなくなります。呼び出し側が手放したノードは、
まだ木の中にあっても解放されてしまいます。

### 別解

テンプレートの `adopt` の形を残す方法です。`Weak` には `take` メソッドはありませんが、`RefCell::take` は
`T: Default` なら使えます。古い `Weak` を返し、代わりに `Weak::new()` を残します。

```rust
pub fn adopt(parent: &Rc<Node>, child: &Rc<Node>) {
    if let Some(old) = child.parent.take().upgrade() {
        old.children.borrow_mut().retain(|c| !Rc::ptr_eq(c, child));
    }
    parent.children.borrow_mut().push(Rc::clone(child));
    *child.parent.borrow_mut() = Rc::downgrade(parent);
}
```

`path` は再帰でも書けます。呼び出しごとに親を 1 つ `upgrade` し、その上のパスを先に作ります。

```rust
pub fn path(n: &Rc<Node>) -> String {
    match n.parent.borrow().upgrade() {
        Some(p) => format!("{}/{}", path(&p), n.name),
        None => n.name.clone(),
    }
}
```
