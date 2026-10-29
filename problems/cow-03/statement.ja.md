コピーは高くつくので、本当に何かを変えるときだけコピーするようにします。行のリストで、複製（clone）が
安価なスナップショットになる型 `Doc` を実装してください。`Doc` はエディタに入っています。

```rust
use std::rc::Rc;

#[derive(Clone)]
pub struct Doc {
    lines: Rc<Vec<String>>,
}

impl Doc {
    pub fn new(lines: &[&str]) -> Doc;
    pub fn line(&self, i: usize) -> Option<&str>;
    pub fn set_line(&mut self, i: usize, text: &str);
    pub fn shares_storage(&self, other: &Doc) -> bool;
}
```

- `new` は、`lines` のコピーを持つ新しい専用の記憶領域で `Doc` を作ります。
- （derive された）`clone` は、記憶領域を共有するスナップショットを作ります。何もコピーしません。
- `line(i)` は `i` 行目（0 始まり）を返します。`i` が範囲外なら `None` です。
- `set_line(i, text)` は `i` 行目を `text` に置き換えます。ほかの `Doc` は決して変わりません。
  記憶領域をまだ存在するほかの `Doc` と共有しているときは、先にこの `Doc` 専用のコピーを作ります。
  共有していなければ、コピーせずにその場で行を置き換えます。
- `shares_storage` は、2 つの `Doc` が同じ記憶領域を使っているかどうかを返します（行が等しいだけでは
  共有ではありません）。

### 例

```rust
let a = Doc::new(&["x", "y"]);
let mut b = a.clone();    // a.shares_storage(&b) は true
b.set_line(0, "z");       // false になる
// a.line(0) == Some("x")、b.line(0) == Some("z")
```
