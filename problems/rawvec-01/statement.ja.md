`Vec<T>` のような可変長配列 `MyVec<T>` を、`std::alloc` と生ポインタを使ってゼロから作ってください。
3 つのフィールドを持つ構造体はエディタに入っています。

```rust
pub struct MyVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self;
    pub fn len(&self) -> usize;
    pub fn capacity(&self) -> usize;
    pub fn push(&mut self, x: T);
    pub fn pop(&mut self) -> Option<T>;
    pub fn get(&self, i: usize) -> Option<&T>;
    pub fn as_slice(&self) -> &[T];
}

impl<T> Drop for MyVec<T> { /* 実装する */ }
```

- 要素を `Vec`、`Box`、`String` などの標準のコレクションに格納してはいけません。
  バッファは `std::alloc`（`alloc`、`realloc`、`dealloc`）を使って自分で確保します。
- `new` は何も確保せずに空の `MyVec` を作ります。容量は `0` です（サイズ 0 の型は例外で、後述します）。
- `len` は要素の数、`capacity` はバッファに入る要素の数です。
- `push` は `x` を末尾に追加します。`MyVec` がいっぱい（`len() == capacity()`）のときは、
  先に容量を増やします。`0` は `4` になり、それ以外は 2 倍になります（`4` → `8` → `16` → ...）。
  容量はこれ以外のときには変わりません。特に、`pop` で容量が減ることはありません。
- `pop` は最後の要素を取り除いて返します。`MyVec` が空なら `None` を返します。
- `get(i)` は添字 `i` の要素を返します。`i >= len()` なら `None` です。
  `as_slice` はすべての要素を順に並べたスライスを返します（要素がなければ空のスライス）。
- `()` のようなサイズ 0 の型 `T` では、`capacity()` は常に `usize::MAX` で、メモリは一切確保しません。
  それ以外の動作は通常と同じです。
- `MyVec` が破棄されるときは、残っている要素をそれぞれちょうど 1 回ずつ（順序は問いません）破棄し、
  バッファを解放します。
- 未定義動作やメモリリークは Miri が検出し、不合格になります。メモリ確保の失敗や容量のオーバーフローは
  テストしません。その場合はパニックして構いません。

### 例

```rust
let mut v = MyVec::new();
// len() は 0、capacity() は 0
for s in ["a", "b", "c", "d", "e"] {
    v.push(String::from(s));
}
// 各 push の後の capacity() は 4, 4, 4, 4, 8
// len() は 5、as_slice() は ["a", "b", "c", "d", "e"]
v.pop(); // Some("e")
v.pop(); // Some("d")
// get(2) は Some("c")、get(3) は None、len() は 3、capacity() は 8

let mut z: MyVec<()> = MyVec::new();
// capacity() は usize::MAX
z.push(());
z.push(());
// len() は 2、capacity() は usize::MAX のまま
z.pop(); // Some(())
// len() は 1
```
