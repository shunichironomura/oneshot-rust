コピーは高くつくので、本当に何かを変えるときだけコピーするようにします。書くときだけコピーする（clone-on-write）
小さな関数と型を 3 つ実装してください。`Doc` はエディタに入っています。

```rust
use std::borrow::Cow;
use std::rc::Rc;

pub fn normalize_ws(s: &str) -> Cow<'_, str>;
pub fn clamp_all(v: &mut Cow<'_, [i32]>, max: i32);

#[derive(Clone)]
pub struct Doc {
    lines: Rc<Vec<String>>,
}

impl Doc {
    pub fn new(lines: &[&str]) -> Doc;
    pub fn line(&self, i: usize) -> Option<&str>;
    pub fn set_line(&mut self, i: usize, text: &str);
    pub fn shares_storage(&self, other: &Doc) -> bool;
    pub fn storage_id(&self) -> usize;
}
```

`normalize_ws` は `s` の空白を整えます。

- 空白とは、ASCII の空白文字 `' '`、`'\t'`、`'\n'`、`'\r'` と改ページ `'\x0C'` のことです
  （`char::is_ascii_whitespace` が真になる文字そのものです）。それ以外の文字は空白ではありません。
  垂直タブ `'\x0B'` や、U+3000 のような ASCII 以外の空白も空白として扱いません。
- 結果は、`s` の両端の空白を取り除き、空白でない 2 文字の間にある空白の連なりをそれぞれ 1 つの `' '` に
  置き換えた文字列です。
- 置き換える連なりが 1 つもない（内側の連なりがどれもちょうど 1 つの `' '` である）ときは、結果は `s` の一部
  そのものです。コピーせずに、`s` のスライスを `Cow::Borrowed` で返してください。それ以外のときは
  `Cow::Owned` を返します。

`clamp_all` は `v` の値を `max` 以下に抑えます。

- `max` より大きい要素はすべて `max` になります。それ以外の要素はそのままです。
- `max` より大きい要素が 1 つもなければ、`v` は何も変わりません。`Cow::Borrowed` は `Borrowed` のままで、
  何もコピーしません。
- そうでなければ、`Cow::Borrowed` は値を抑えた `Cow::Owned` になります。借用していたスライス自体は
  変わりません。
- `Cow::Owned` は `Owned` のままで、その `Vec` をその場で書き換えます（新しい `Vec` は作りません）。

`Doc` は行のリストで、複製（clone）は安価なスナップショットになります。

- `new` は、`lines` のコピーを持つ新しい専用の記憶領域で `Doc` を作ります。
- （derive された）`clone` は、記憶領域を共有するスナップショットを作ります。何もコピーしません。
- `line(i)` は `i` 行目（0 始まり）を返します。`i` が範囲外なら `None` です。
- `set_line(i, text)` は `i` 行目を `text` に置き換えます。ほかの `Doc` は決して変わりません。
  記憶領域をまだ存在するほかの `Doc` と共有しているときは、先にこの `Doc` 専用のコピーを作ります。
  共有していなければ、コピーせずにその場で行を置き換えます。
- `i` が範囲外なら、`set_line` はパニックし、`Doc` は元のままです。特に、記憶領域のコピーは作りません。
- `shares_storage` は、2 つの `Doc` が同じ記憶領域を使っているかどうかを返します（行が等しいだけでは
  共有ではありません）。
- `storage_id` は、`lines` が指している `Vec<String>` のアドレスを `usize` で返します。したがって
  `a.storage_id() == b.storage_id()` と `a.shares_storage(&b)` の真偽は常に一致し、`Doc` をその場で
  書き換えている間は値が変わりません。

### 例

```rust
normalize_ws("a b")       // Borrowed("a b")
normalize_ws("  hi\n")    // Borrowed("hi")。入力のスライス
normalize_ws("a  b\tc ")  // Owned("a b c")

let data = [1, 5, 3];
let mut v = Cow::Borrowed(&data[..]);
clamp_all(&mut v, 9);     // Borrowed([1, 5, 3]) のまま
clamp_all(&mut v, 4);     // Owned([1, 4, 3]) になる。data は変わらない

let a = Doc::new(&["x", "y"]);
let mut b = a.clone();    // a.shares_storage(&b) は true
b.set_line(0, "z");       // false になる
// a.line(0) == Some("x")、b.line(0) == Some("z")
```
