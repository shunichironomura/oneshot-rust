リングバッファは、固定長の配列に最新の要素を保持します。容量 `N` を const ジェネリック引数に持ち、
最大 `N` 個の要素を先入れ先出しで保持するキュー `Ring<T, N>` を実装してください。
構造体、定数 `CAPACITY` とメソッドのシグネチャはエディタに入っています。

```rust
pub struct Ring<T, const N: usize> {
    buf: [Option<T>; N],
    start: usize,
    len: usize,
}

impl<T, const N: usize> Ring<T, N> {
    pub const CAPACITY: usize = N;

    pub fn new() -> Self;
    pub fn push(&mut self, x: T) -> Option<T>;
    pub fn pop(&mut self) -> Option<T>;
    pub fn len(&self) -> usize;
    pub fn is_full(&self) -> bool;
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_;
}
```

- `new` は空のリングを返します。
- `push(x)` は `x` を最も新しい要素として追加します。リングが満杯でなかったときは `None` を返します。
  満杯だった（要素を `N` 個持っていた）ときは、最も古い要素を取り除いて `Some` で返します。
  したがって、リングの要素数は `N` のままです。
- `pop` は最も古い要素を取り除いて返します。リングが空なら `None` を返します。
  このときリングは空のままで、引き続き使えます。
- `len` は要素数を返し、`is_full` は要素数が `N` かどうかを返します。
- `iter` は、要素への参照を古いものから新しいものの順に返すイテレータを返します。リングは変更しません。
- `Ring::<T, N>::CAPACITY` は `N` です（すでに書いてあります）。
- `T` はどんな型でもかまいません。`Copy`、`Clone`、`Default`、`Debug` のどれも実装していない型も含みます
  （ハーネスはそのような型を使います）。`T` に境界を追加しないでください。
- `Ring` のフィールドは変えないでください。要素は、`Vec` や `VecDeque` のような
  ヒープ上のコレクションではなく、構造体の中にある配列 `buf` に格納します。
  `start` と `len` は、`buf` の中での最も古い要素の位置と、要素数を表すためのフィールドです。
- `N` は 1 以上です。

### 例

```rust
let mut r: Ring<i32, 2> = Ring::new();
r.push(1); // None
r.push(2); // None
r.push(3); // Some(1)：満杯だったので、最も古い要素 1 が追い出される
// このとき r.len() は 2、r.is_full() は true で、r.iter() は &2、&3 の順に返す。
r.pop();   // Some(2)
// Ring::<i32, 3>::CAPACITY は 3。
```
