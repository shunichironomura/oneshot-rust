バッファのうち `len` より後ろは**未初期化のメモリ**です。落とし穴のほとんどは、そこにすでに `T` が
入っているかのように扱うことから生まれます。

- `push` では `ptr::write`（ここでは `NonNull::write`）で値を書き込みます。代入 `*slot = x` は、
  先にスロットにある古い値を破棄します。`String` なら、ゴミをポインタとして読んで解放しようとするので、
  Miri は未初期化メモリの読み出しとして報告します。`pop` はその逆で、`len` を減らしてから `ptr::read` で
  値を取り出します。そのスロットはもう数えられないので、2 回破棄されることはありません。
- バッファのサイズとアラインメントは `Layout::array::<T>(cap)` で求めます。最初のバッファは `alloc` で
  確保します。その後の `realloc(ptr, old_layout, new_size)` に渡すレイアウトは、新しいものではなく
  **確保したときの**レイアウトです。`dealloc` にも同じレイアウトが必要です。結果が null なら確保に
  失敗しているので、`handle_alloc_error` で報告します。
- 空の `MyVec` は `NonNull::dangling()` を持ちます。null ではなくアラインメントも正しいので、長さ 0 の
  スライスにはこれで十分ですが、どの確保領域も指していません。`cap == 0` のとき、`Drop` でこれを
  `dealloc` してはいけません。
- `Drop` では、初期化済みの部分を `ptr::drop_in_place` で 1 つのスライスとして破棄してから、
  バッファを解放します。

用意されている `get` には専用の `unsafe` は要りません。`as_slice` が `slice::from_raw_parts` でスライスを作るので、
範囲のチェックはスライスの `get` に任せられます。

### 別解

`grow` では `realloc` の代わりに、新しいバッファを確保し、`ptr::copy_nonoverlapping` で要素を移して、
古いバッファを解放することもできます（バイトのコピーで値は移動し、古い領域を解放しても何も
破棄されません）。`Drop` は、空になるまで `pop` を繰り返して、要素を 1 つずつ取り出して破棄する
こともできます。

```rust
fn grow(&mut self) {
    let new_cap = if self.cap == 0 { 4 } else { 2 * self.cap };
    let new_layout = Layout::array::<T>(new_cap).expect("capacity overflow");
    // SAFETY: T is not zero-sized and new_cap > 0.
    let new_ptr = unsafe { alloc::alloc(new_layout) }.cast::<T>();
    let Some(new_ptr) = NonNull::new(new_ptr) else {
        alloc::handle_alloc_error(new_layout);
    };
    if self.cap > 0 {
        // SAFETY: the old buffer holds `len` initialized values and the new one is larger.
        // After the copy they belong to the new buffer; the old one is freed with its own
        // layout, which does not drop anything.
        unsafe {
            ptr::copy_nonoverlapping(self.ptr.as_ptr(), new_ptr.as_ptr(), self.len);
            alloc::dealloc(self.ptr.as_ptr().cast(), Layout::array::<T>(self.cap).unwrap());
        }
    }
    self.ptr = new_ptr;
    self.cap = new_cap;
}
```

```rust
impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
        if self.cap > 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            // SAFETY: `ptr` was allocated with exactly this layout.
            unsafe { alloc::dealloc(self.ptr.as_ptr().cast(), layout) };
        }
    }
}
```
