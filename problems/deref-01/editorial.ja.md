`SortedVec<T>` をスライスのように読めるようにしているのは、`Target = [T]` の `Deref` です。`deref` の中の
`&self.items` は `&Vec<T>` ですが、参照外し型強制（deref coercion）でシグネチャどおりの `&[T]` に
変換されます。呼び出し側では残りが自動で手に入ります。`sv.len()` や `sv.windows(2)` のようなメソッド呼び出しは、
`SortedVec` にその名前のメソッドがなければ `SortedVec<T>` を `[T]` に自動で参照外しします。`sv[0]` のような
インデックスも同じ自動参照外しを通り、`&sv` はスライスを受け取る関数に渡すと `&[T]` に型強制されます。
`DerefMut` がないので、これらはすべて読み取り専用です。`sv.swap(0, 1)`、`sv.sort_by(...)`、`sv[0] = x` は
`&mut [T]` が必要なのでコンパイルが通りません。中身を変えられるのは `insert` と `remove` だけになり、
順序が崩れることはありません。

スライスのメソッドより `SortedVec` 自身のメソッドが優先されます。`impl` の中で
`self.insert(index, value)` と書くと、`Vec::insert` ではなく（引数が 1 つの）`SortedVec::insert` が
呼ばれるので、`self.items.insert(...)` や `self.items.remove(...)` と書きます。

等しい要素があるとき、`binary_search` は一致する位置の**どれか**を返し、どれを返すかはドキュメントで
決められていません（今の実装はたまたま最後のものを返すので、これを使った `remove` は違う要素を取り除きます）。
代わりに `partition_point(pred)` を使うと、`pred` が偽になる最初の位置が得られます。

- `partition_point(|x| x <= &value)` は `value` より大きい最初の要素、つまり等しい要素すべての直後の位置です。
  `insert` が `value` を入れる位置になります。
- `partition_point(|x| x < value)` は `value` 以上の最初の要素で、等しい要素があればそのいちばん左のものです。
  `self.items.get(index) == Some(value)` でそれを確かめられ、`index == len` の場合も扱えます。

これらのクロージャの `x` は `&T` なので、`&value` と比べます（`*x <= value` でも構いません。比較演算子は
オペランドを参照で受け取ります）。`|&x| x <= value` は参照から `T` をムーブしようとするので、`i32` のような
`Copy` 型でしか使えません。`|x| x <= value` は `&T` と `T` を比べることになり、型が合いません。

`sort` は安定ですが、`sort_unstable` は等しい要素の順序を入れ替えることがあります
（長めの入力では実際に入れ替わります）。ベクタをその場で並べ替えるには、`from` の引数を `mut items` と
宣言する必要があります。

### 別解

線形に探すほうが簡単です。`Vec::insert` と `Vec::remove` はどのみち後ろの要素をずらすので、
どちらのメソッドも O(n) のままです。

```rust
pub fn insert(&mut self, value: T) -> usize {
    let index = self
        .items
        .iter()
        .position(|x| x > &value)
        .unwrap_or(self.items.len());
    self.items.insert(index, value);
    index
}

pub fn remove(&mut self, value: &T) -> bool {
    match self.items.iter().position(|x| x == value) {
        Some(index) => {
            self.items.remove(index);
            true
        }
        None => false,
    }
}
```

`Equal` を決して返さない比較を渡すと、`binary_search_by` は境界の位置を `Err` で返します。
`then(Ordering::Less)` は等しい要素を小さいものとして扱うので、挿入位置が得られます
（`use std::cmp::Ordering;` が必要です）。`then(Ordering::Greater)` にすると、等しい要素のいちばん左の
位置になります。

```rust
pub fn insert(&mut self, value: T) -> usize {
    let index = self
        .items
        .binary_search_by(|x| x.cmp(&value).then(Ordering::Less))
        .unwrap_err();
    self.items.insert(index, value);
    index
}
```

`from` は要素を 1 つずつ `insert` しても書けます。それぞれの要素はすでにある等しい要素の後ろに入るので、
等しい要素どうしは入力の順序を保ちます。ただし計算量は O(n log n) ではなく O(n²) になります。

```rust
fn from(items: Vec<T>) -> Self {
    let mut sorted = SortedVec::new();
    for item in items {
        sorted.insert(item);
    }
    sorted
}
```
