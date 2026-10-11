`less` は呼び出し側のコードなので、パニックすることがあります。そのときパニックは `less` を呼んだその場所から
`insert_sorted` の外へ巻き戻り（unwind）、ベクタはその瞬間の状態のまま残って、あとでいつもどおりドロップされます。
テンプレートでは、ループの間ずっとその状態が壊れています。

- `ptr::read` はビット単位のコピーを作るので、`x` を `tmp` と最後の要素の 2 か所が所有しています。
- `ptr::copy_nonoverlapping` をするたびに、1 つの要素が 2 つの場所に入っています。

最初の呼び出しで `less` がパニックすると、巻き戻りで `tmp` がドロップされ、あとで `v` をドロップするときに `x` が
もう一度ドロップされます。ずらした後でパニックすると、`v` のドロップでずらした要素が 2 回ドロップされます。
`String` なら二重解放で、Miri は `v` のドロップ時に解放済みメモリの使用（use-after-free）を報告します。

これは**例外安全性**（Rustonomicon の「Exception Safety」）の問題です。自分で制御できないコード（クロージャや、
`Ord::cmp`・`Clone::clone`・`Drop::drop` のようなトレイトのメソッド）が動くときは、いつでも、すべてが
ドロップしても安全な状態になっていなければなりません。`tmp` を `ManuallyDrop` で包むだけでは足りません。
最初の呼び出しでのパニックは直りますが、ずらした後は、やはり 1 つの要素が 2 つの場所に入っています。

模範解答は 2 段階で処理します。まず見るだけです。`partition_point` は二分探索をするだけで、`v` を変更しません。
ここで `less` がパニックしても `v` は元のままで、まだ関数のローカル変数である `x` は、巻き戻りの間に 1 回だけ
ドロップされます。次に `Vec::insert` で `v` を変更します。`Vec::insert` は値を移動するだけで、呼び出し側のコードを
呼びません。

述語 `!less(&x, e)` は、`x` より大きくない要素で `true` になります。整列済みのベクタではそれらの要素が前に
集まっているので、`partition_point` は `x` と等しい最後の要素の次の位置を返します。`|e| less(e, &x)` にすると、
等しい要素の**前**の位置になってしまいます。ここで `binary_search_by` に頼ってはいけません。等しい要素が
複数あるとき、そのうちのどれを返してもよいとドキュメントに書かれています。

`std::panic::catch_unwind` でパニックを捕まえても解決にはなりません。パニックは呼び出し元まで伝わる必要があります。

### 別解

`x` を末尾に追加してから、挿入ソートのように交換で前へ移動していく方法です。`swap` は 1 回 1 回が完結した操作
なので、`less` が動くときベクタはいつも有効で、パニックしたら `x` はその場所に残るだけです。`unsafe` は要りません。

```rust
pub fn insert_sorted<T, F: FnMut(&T, &T) -> bool>(v: &mut Vec<T>, x: T, mut less: F) {
    v.push(x);
    let mut i = v.len() - 1;
    while i > 0 && less(&v[i], &v[i - 1]) {
        v.swap(i, i - 1);
        i -= 1;
    }
}
```

2 段階の解法では、`less` を呼ぶ回数は増えますが、線形探索でもかまいません。
`let pos = v.iter().position(|e| less(&x, e)).unwrap_or(v.len());` です。

テンプレートのずらし方を残したまま、空いた場所を所有するガードでパニックに強くすることもできます。標準ライブラリが
`BinaryHeap` で使っている `Hole` と同じ考え方です。ガードの `Drop` が値を空いた場所に書き込みます。`Drop` は
巻き戻りの間にも実行されるので、関数をどのように抜けてもベクタは有効になります。

```rust
use std::mem::ManuallyDrop;
use std::ptr;

/// While a `Hole` is alive, the slot `pos` of `v` is logically empty, and the value that belongs
/// there is kept in `value`. Dropping the hole, normally or while unwinding, moves the value back
/// into the slot, so `v` is valid again.
struct Hole<'a, T> {
    v: &'a mut [T],
    value: ManuallyDrop<T>,
    pos: usize,
}

impl<T> Drop for Hole<'_, T> {
    fn drop(&mut self) {
        unsafe {
            ptr::copy_nonoverlapping(&*self.value, self.v.as_mut_ptr().add(self.pos), 1);
        }
    }
}

pub fn insert_sorted<T, F: FnMut(&T, &T) -> bool>(v: &mut Vec<T>, x: T, mut less: F) {
    v.push(x);
    let last = v.len() - 1;
    let value = ManuallyDrop::new(unsafe { ptr::read(&v[last]) });
    let mut hole = Hole { v: v.as_mut_slice(), value, pos: last };
    while hole.pos > 0 && less(&hole.value, &hole.v[hole.pos - 1]) {
        let p = hole.v.as_mut_ptr();
        unsafe { ptr::copy_nonoverlapping(p.add(hole.pos - 1), p.add(hole.pos), 1) };
        hole.pos -= 1;
    }
    // `hole` is dropped here and writes the value into its final slot.
}
```
