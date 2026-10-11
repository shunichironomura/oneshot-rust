`SortedVec<T>` は、要素がいつも昇順に並んでいるベクタです。中身を変えられるのは自身のメソッドだけですが、
スライスと同じように読むことはできます。`[T]` に参照外し（deref）できるので、`len`、`iter`、`first`、
`contains`、`windows` のような読み取り専用のスライスのメソッドや、`sv[0]` のようなインデックスが
そのまま使え、`&[T]` を受け取る関数に `&sv` を渡せます。これを実装してください（構造体と `impl` ブロックは
エディタに入っています）。

```rust
pub struct SortedVec<T> {
    items: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    pub fn new() -> Self;
    pub fn insert(&mut self, value: T) -> usize;
    pub fn remove(&mut self, value: &T) -> bool;
}

impl<T: Ord> From<Vec<T>> for SortedVec<T> {
    fn from(items: Vec<T>) -> Self;
}

impl<T> SortedVec<T> {
    pub fn into_vec(self) -> Vec<T>;
}

impl<T> Deref for SortedVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T];
}
```

- `new` は空の `SortedVec` を返します。
- `insert` は `value` を、それと等しいすべての要素の後ろ、それより大きいすべての要素の前に入れて、
  `value` が入った位置のインデックスを返します。
- `remove` は `value` と等しい要素のうち、いちばん左のもの 1 つだけを取り除いて `true` を返します。
  `value` と等しい要素がなければ、何も変えずに `false` を返します。
- `SortedVec::from(v)` は `v` の要素を持つ `SortedVec` を返します。等しい要素どうしは `v` での順序を保ちます。
- `into_vec` は要素を `Vec<T>` として、`deref` はスライスとして返します。どちらも昇順です。
- `T` は `Ord` を実装した任意の型です。ハーネスは `i32`、`String` と、独自の型 `Key(u8, char)` を使います。
  `Key` は数だけで順序が決まるので、`Key(2, 'a')` と `Key(2, 'b')` は等しくなりますが、文字で見分けられます。
- `items` は非公開のままにし、`DerefMut` は実装しないでください。要素をその場で書き換えられると、
  順序が崩れるおそれがあるからです。

### 例

```rust
let mut sv = SortedVec::from(vec![3, 1, 2]); // &*sv は [1, 2, 3]
sv.insert(2)   // 2。sv は [1, 2, 2, 3] になる
sv.remove(&2)  // true。sv は [1, 2, 3] になる
sv.remove(&5)  // false
sv.len()       // 3
sv[0]          // 1
```

```rust
let mut sv = SortedVec::from(vec![Key(2, 'a'), Key(1, 'x'), Key(3, 'y')]);
// 文字の並びは x, a, y
sv.insert(Key(2, 'b'))   // 2。文字の並びは x, a, b, y になる
sv.remove(&Key(2, 'z'))  // true。文字の並びは x, b, y になる
```
