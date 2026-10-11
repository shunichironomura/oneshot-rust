`set_len(n)` は「先頭の `n` 個の要素は初期化済み」と約束する操作ですが、`with_capacity` が返す
バッファは初期化されていません。それ以降、その領域に触れる処理はどれも、ゴミを `T` として扱います。

- `v[i] = f(i)` は代入で、代入は新しい値を書き込む前に**古い値をドロップ**します。`i32` なら
  何も起きないので、テンプレートは数値では動きます。`String` の場合は、ゴミのポインタと容量を読んで
  解放しようとするので、Miri は未初期化メモリの読み出しとして報告します。
- `ptr::write`（`v.as_mut_ptr().add(i).write(f(i))`）で書き込めば何もドロップしませんが、それだけでは
  足りません。`f` がパニックすると巻き戻しで `v` がドロップされ、長さがすでに `n` なので、
  まだ書き込んでいない要素までドロップされます。
- 長さをループの後で設定すればこれは避けられますが、パニックしたときの `v` は長さ 0 のままなので
  何もドロップせず、それまでに書き込んだ値がリークします。
- 先頭から順に埋めていくベクタなら、書き込むたびに長さを 1 ずつ増やすのが定番の直し方で、
  `push` がまさにそうしています。しかしこの問題では埋める順番がばらばらなので、書き込み済みの値が
  先頭に並んでいるとは限りません。

`f` がパニックしたとき、それまでに作った値はあちこちの位置に散らばっていて、どれがそうなのかを
誰かが把握していなければなりません。模範解答はそれを型に任せます。`None` を並べた
`Vec<Option<T>>` から始めて、`Some(f(i))` を入れていきます。`slots` は普通のローカル変数なので、
`f` がパニックすれば巻き戻しでドロップされ、中にある値を過不足なくドロップします。
`slots[i]` への代入も問題ありません。ドロップされるのは `None` だけだからです。最後にはどの要素も
`Some` なので、`into_iter().map(...).collect()` で値を `Vec<T>` に移します。`unsafe` は
要りません。代償は、各値を包む `Option`（`String` や `Box` などの型なら余分な領域すら
使いません）と、最後にそれを外す 1 回の走査です。

`vec![None; n]` には注意してください。要素を複製するので `Option<T>: Clone`、つまり `T: Clone` が
必要ですが、`build` にはその境界がありません。`order.iter().map(|_| None).collect()` や
`iter::repeat_with(|| None).take(n).collect()` なら、複製せずに `None` を作れます。

### 別解

`(インデックス, 値)` の組を呼び出し順に集めてから、インデックスで並べ替える方法です。これも
そのままでパニックに強いコードになっています。`f` がパニックすると、`collect` が作っていた
ベクタが中の組ごとドロップされるからです。

```rust
pub fn build<T>(order: &[usize], mut f: impl FnMut(usize) -> T) -> Vec<T> {
    let mut pairs: Vec<(usize, T)> = order.iter().map(|&i| (i, f(i))).collect();
    pairs.sort_unstable_by_key(|&(i, _)| i);
    pairs.into_iter().map(|(_, value)| value).collect()
}
```

`unsafe` を使うなら、長さを 0 のままにした `Vec` の空き容量に値を書き込み、どの位置に値が
あるか（`order[..written]`）を知っている**ガード**を持っておきます。`f` がパニックすると、
巻き戻しでガードがドロップされてそれらの値だけをドロップし、その後で `v` がバッファを解放します。
すべての位置に書き込んだら、`mem::forget` でガードを解除し、`set_len(n)` で値を `v` に引き渡します。
`v` の型注釈に注意してください。これがないと、その時点ではまだ要素の型がわからないので、
`guard.v.as_mut_ptr().add(i)` がコンパイルできません。

```rust
use std::mem;

/// Owns the values written so far into the buffer of `v`, whose length is still 0: they are in
/// the slots `order[..written]`. Dropping the guard drops exactly those values.
struct Written<'a, T> {
    v: &'a mut Vec<T>,
    order: &'a [usize],
    written: usize,
}

impl<T> Drop for Written<'_, T> {
    fn drop(&mut self) {
        let p = self.v.as_mut_ptr();
        for &i in &self.order[..self.written] {
            unsafe { p.add(i).drop_in_place() };
        }
    }
}

pub fn build<T>(order: &[usize], mut f: impl FnMut(usize) -> T) -> Vec<T> {
    let n = order.len();
    let mut v: Vec<T> = Vec::with_capacity(n);
    let mut guard = Written { v: &mut v, order, written: 0 };
    for &i in order {
        let value = f(i);
        // `write` does not drop the old (uninitialized) contents of the slot.
        unsafe { guard.v.as_mut_ptr().add(i).write(value) };
        guard.written += 1;
    }
    // Every slot holds a value now: disarm the guard and hand the values over to `v`.
    mem::forget(guard);
    unsafe { v.set_len(n) };
    v
}
```

`written` は実際に書き込み済みの値だけを数えなければなりません。`f(i)` が返る前に増やすと、
ガードが空の位置をドロップしてしまいます。また、このコードが健全なのは `order` が並べ替えだから
です。範囲外のインデックスがあればバッファの外に書き込み、同じインデックスが 2 回あればどこかの位置が
未初期化のまま残ります。実際に公開する関数なら、先に `order` を検査する必要があります。
