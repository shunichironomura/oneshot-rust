`Ring` には見た目より難しいところが 2 つあります。配列の作り方と、要素の出し入れです。

`new` では `[None; N]` が使えません。繰り返し式 `[x; N]` は `x` を各要素にコピーするので、
`x` は `Copy` でなければならず、`Option<T>` が `Copy` になるのは `T` が `Copy` のときだけです。
ジェネリックなコードは、ありうるすべての `T` についてまとめて一度だけ型検査されるので、
例で `i32` を使っていても関係ありません。コンパイラは `[None; N]` を E0277 で拒否します。
コンパイラの提案どおりに境界 `T: Copy` を追加しても、`String` を使うハーネスの側にエラーが移るだけです。
例外は定数です。インライン `const` ブロック（Rust 1.79 で安定化）はコピーされるのではなく、
要素ごとに評価し直されるので、`[const { None }; N]` はどんな `T` でも使えます。
`Default::default()` もうまくいきません。配列が `Default` を実装しているのは長さ 32 までで、
任意の `N` ではないからです。

`self.buf[i]` は配列の中の場所を指します。`let x = self.buf[i];` は要素をムーブして
配列に穴を空けてしまうので、エラーになります（E0507）。代わりに `Option` のメソッドで値を入れ替えます。
`take()` は `None` を残して元の値を返し、`replace(x)` は `Some(x)` を入れて元の値を返します。
`pop` は最も古い要素を取り出し、満杯のリングへの `push` は、同じ場所で最も古い要素を
新しい要素に置き換えます。

残りは `N` を法とする添字の計算です。最も古い要素は `start` に、その `i` 個後の要素は
`(start + i) % N` に、最初の空きは `(start + len) % N` にあります。`pop` や追い出しの後は、
`start` を 1 つ進めます。`iter` は添字 `0..len` を `buf` の中への参照に変換するので、
イテレータはリングを借用します。`+ '_` はこのことを表しています（エディション 2024 では、
書かなくても `impl Trait` の返り値型はこのライフタイムをキャプチャします）。

### 別解

`std::array::from_fn` は添字ごとにクロージャを呼ぶので、これも `Copy` を必要としません。

```rust
pub fn new() -> Self {
    Ring { buf: std::array::from_fn(|_| None), start: 0, len: 0 }
}
```

インライン `const` ブロックができる前は、名前のついた定数を使うのが定番でした。定数アイテムへのパスも、
繰り返し式のオペランドに使えます。

```rust
impl<T, const N: usize> Ring<T, N> {
    pub const CAPACITY: usize = N;
    const EMPTY: Option<T> = None;

    pub fn new() -> Self {
        Ring { buf: [Self::EMPTY; N], start: 0, len: 0 }
    }

    // ほかのメソッドは同じです。
}
```

`push` は、`pop` を使って場所を空けても書けます。

```rust
pub fn push(&mut self, x: T) -> Option<T> {
    let evicted = if self.is_full() { self.pop() } else { None };
    self.buf[(self.start + self.len) % N] = Some(x);
    self.len += 1;
    evicted
}
```

空いている場所にはどれも `None` が入っているので、`iter` は配列を `start` から最後まで、
続けて先頭からたどり、`flatten` で `None` を読み飛ばしても書けます
（`&Option<T>` は `&T` を 0 個か 1 個返すイテレータになります）。

```rust
pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
    let (wrapped, oldest_first) = self.buf.split_at(self.start);
    oldest_first.iter().chain(wrapped).flatten()
}
```
