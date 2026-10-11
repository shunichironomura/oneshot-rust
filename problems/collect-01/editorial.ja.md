トレイトの実装そのものは短く済みます。`FromIterator` は空の袋から始めて `Extend<T>` に
任せ、`Extend<T>` は `*self.counts.entry(x).or_insert(0) += 1` で個数を増やします。`Extend<&'a T>` は
`Vec` にもある `Copy` な要素向けの実装と同じで、各要素をコピーして
`self.extend(iter.into_iter().copied())` のように `Extend<T>` を再利用できます。`copied()` は `T` を
返すので、ここでは `Extend<T>` が選ばれます。

本当の仕事はイテレータ型のほうです。`type IntoIter` には書き下せる型が必要ですが、クロージャを
持つ `map` や `flat_map` のようなアダプタの型には名前を付けられません（安定版の Rust では関連型に
`impl Trait` を使えません）。そこで、`Iter` と `IntoIter` はマップ自身のイテレータを包む構造体に
します。

- `Iter` は `btree_map::Iter<'a, T, usize>` を包みます。これは `(&T, &usize)` を返すので、
  `|(x, &n)| (x, n)` で `(&T, usize)` に変えます。`next_back` もマップのものが使えます。パスは
  `btree_map::Iter` と書いてください。`std::collections::btree_map::Iter` を `use` すると、自分の
  `Iter` と名前がぶつかります（E0255）。
- `IntoIter` は `(要素, 個数)` の各エントリを、遅延評価で両端から、コピーの並びに展開しなければ
  なりません。`iter::repeat_n(x, n)` は `n` 個のコピーを返しますが、元の値を最後に返すので
  クローンは `n - 1` 回だけです。`iter::repeat(x).take(n)` は `n` 回クローンするので回数の制約を
  守れず、`into_iter` の中でコピーを `Vec` に集めると最初に全部クローンしてしまいます。

この展開を両端から行うのは `flat_map` がすでにやってくれるので、障害は `FlatMap<I, U, F>` の
クロージャの型 `F` だけです。何もキャプチャしないクロージャは関数ポインタに型強制できるので、
フィールドは `F = fn((T, usize)) -> RepeatN<T>` として宣言できます。このとき構造体自体に
`T: Clone` が必要になります。`FlatMap` は `U: IntoIterator` を要求し、`RepeatN<T>` がイテレータに
なるのは `T: Clone` のときだけだからです。

この型強制が起きるのは、フィールドの型が期待される場所にクロージャを直接書いているからです。
先に `let` でイテレータを変数に入れると、型はクロージャから推論されてフィールドと合わなくなります
（E0308）。その場合はクロージャを明示的にキャストしてください。
`.flat_map((|(x, n)| iter::repeat_n(x, n)) as fn(_) -> _)` のように書けます。

### 別解

どちらも模範解答の `IntoIter` の部分を置き換えるものです（`use` 宣言もそれに合わせて調整してください）。

両端それぞれに `RepeatN` を 1 つずつ持たせて、手で展開する方法です（`RepeatN::default()` は空の
`RepeatN` です）。真ん中のエントリがなくなったら、反対側の端が取り出し始めたコピーの残りを
引き継がなければなりません。そうしないと、`next` と `next_back` を混ぜたときに項目が抜けます。

```rust
/// Iterator that moves the elements out of a bag.
pub struct IntoIter<T> {
    groups: btree_map::IntoIter<T, usize>,
    // The copies of the entry taken from each end (an empty `RepeatN` when there is none).
    front: RepeatN<T>,
    back: RepeatN<T>,
}

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            if let Some(x) = self.front.next() {
                return Some(x);
            }
            match self.groups.next() {
                Some((x, n)) => self.front = iter::repeat_n(x, n),
                // Nothing left in the middle: finish what the back end started.
                None => return self.back.next(),
            }
        }
    }
}

impl<T: Clone> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        loop {
            if let Some(x) = self.back.next_back() {
                return Some(x);
            }
            match self.groups.next_back() {
                Some((x, n)) => self.back = iter::repeat_n(x, n),
                None => return self.front.next_back(),
            }
        }
    }
}

impl<T: Clone> IntoIterator for Bag<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            groups: self.counts.into_iter(),
            front: RepeatN::default(),
            back: RepeatN::default(),
        }
    }
}
```

`repeat_n` を使わずに、両端それぞれで「要素」と「まだ返すコピーの数」を持つ方法です。残りが
2 個以上ならクローンし、最後の 1 個は要素そのものをムーブして返します。

```rust
/// Iterator that moves the elements out of a bag.
pub struct IntoIter<T> {
    groups: btree_map::IntoIter<T, usize>,
    // An element and how many of its copies are still to be handed out, at each end.
    front: Option<(T, usize)>,
    back: Option<(T, usize)>,
}

/// Hands out one copy: a clone while more than one is left, and the original last.
fn take_one<T: Clone>(slot: &mut Option<(T, usize)>) -> Option<T> {
    let (x, n) = slot.take()?;
    if n > 1 {
        let copy = x.clone();
        *slot = Some((x, n - 1));
        Some(copy)
    } else {
        Some(x)
    }
}

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.front.is_none() {
            self.front = self.groups.next().or_else(|| self.back.take());
        }
        take_one(&mut self.front)
    }
}

impl<T: Clone> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.back.is_none() {
            self.back = self.groups.next_back().or_else(|| self.front.take());
        }
        take_one(&mut self.back)
    }
}

impl<T: Clone> IntoIterator for Bag<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter { groups: self.counts.into_iter(), front: None, back: None }
    }
}
```
