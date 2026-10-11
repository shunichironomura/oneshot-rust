素直に書いた次の本体はコンパイルが通りません。

```rust
let (run, tail) = self.rest.split_at_mut(len);
self.rest = tail;
Some(run)
```

`next` が受け取るのは `&mut self` だけで、この借用はこの呼び出しの間の短いライフタイム `'1` しか
続きません。これを通しては `self.rest` を**再借用**することしかできず、再借用は経由した借用より
長くは生きられません。`split_at_mut` が返すのは 2 つの `&'1 mut [T]` で、`&'a mut [T]` ではない
ので、rustc は「method was supposed to return data with lifetime `'a` but it is returning data
with lifetime `'1`」と報告します。この規則が `&mut` の唯一性を守っています。そうでなければ、
`self.rest` がまだ指している要素への `&'a mut` が呼び出し側に渡ってしまいます。すぐ後で
`self.rest` を上書きすることまでは、コンパイラは考慮しません。`let rest = self.rest;` で
スライスを取り出すのもだめです。`&mut [T]` は `Copy` ではなく、`self` は借用しているだけだからです。
（`Copy` である `&'a [T]` のイテレータなら、素直な本体のままで動きます。）

解決策は、`&'a mut [T]` を**値として** `self` から取り出し、代わりに有効な値を置いておくことです。
`std::mem::take(&mut self.rest)` がまさにそれをします。`&mut [T]` は `Default` を実装していて、
既定値は空のスライスです。空のスライスは何も借用しないので、どんなライフタイムにもなれます。
これで関数は `&'a mut [T]` である `rest` を所有します。`split_at_mut` はそれを消費して、重ならない
2 つの `&'a mut [T]` を返し、返さない側を `self.rest` に戻します。`next_back` は反対の端から同じことを
します。Rustonomicon の「Splitting Borrows」で、スライスの `IterMut` を `unsafe` なしで書いている
方法と同じです。

連なりの長さは、take の前に短い共有借用を通して求めます。1 から数え始め、隣どうし（`windows(2)`）を
比べれば、連なりが空になることはありません。`iter().take_while(|x| *x == &rest[0])` で数えても
同じように見えますが、先頭の要素を自分自身とも比べてしまいます。NaN だと 0 個と数え、
イテレータは空の連なりを永遠に返し続けます。`take_while` の代わりに `filter` を使うと、離れた位置にある
等しい値まで数えてしまいます。

### 別解

`std::mem::replace(&mut self.rest, &mut [])` は、空のスライスを明示的に書いた同じ方法です。
`position` と `rposition` で、等しくない隣どうしのうち最初と最後のものを探せます。

```rust
fn next(&mut self) -> Option<&'a mut [T]> {
    let rest = mem::replace(&mut self.rest, &mut []);
    if rest.is_empty() {
        return None;
    }
    // 連なりは、等しくない最初の隣どうしのところで終わる。
    let len = rest.windows(2).position(|w| w[0] != w[1]).map_or(rest.len(), |i| i + 1);
    let (run, tail) = rest.split_at_mut(len);
    self.rest = tail;
    Some(run)
}

fn next_back(&mut self) -> Option<&'a mut [T]> {
    let rest = mem::replace(&mut self.rest, &mut []);
    if rest.is_empty() {
        return None;
    }
    // 最後の連なりは、等しくない最後の隣どうしの直後から始まる。
    let start = rest.windows(2).rposition(|w| w[0] != w[1]).map_or(0, |i| i + 1);
    let (head, run) = rest.split_at_mut(start);
    self.rest = head;
    Some(run)
}
```

問題が起きるのは `&mut self` の向こう側にあるときだけです。**ローカル変数**の `&'a mut [T]` なら、
そのまま分けて上書きできます。ほかからは触れられず、`rest = tail` と代入しても要素には影響しない
からです。そこで、コンストラクタで最初にすべて分けてしまい、イテレータは `VecDeque` から取り出す
だけにすることもできます。その代わり、メモリの確保が必要で、最初の連なりを返す前にスライス全体を
なめることになります。

```rust
use std::collections::VecDeque;

pub struct RunsMut<'a, T> {
    runs: VecDeque<&'a mut [T]>,
}

pub fn runs_mut<T: PartialEq>(v: &mut [T]) -> RunsMut<'_, T> {
    let mut runs = VecDeque::new();
    let mut rest = v;
    while !rest.is_empty() {
        let len = 1 + rest.windows(2).take_while(|w| w[0] == w[1]).count();
        let (run, tail) = rest.split_at_mut(len);
        runs.push_back(run);
        rest = tail;
    }
    RunsMut { runs }
}

impl<'a, T: PartialEq> Iterator for RunsMut<'a, T> {
    type Item = &'a mut [T];

    fn next(&mut self) -> Option<&'a mut [T]> {
        self.runs.pop_front()
    }
}

impl<'a, T: PartialEq> DoubleEndedIterator for RunsMut<'a, T> {
    fn next_back(&mut self) -> Option<&'a mut [T]> {
        self.runs.pop_back()
    }
}
```

最後に、標準ライブラリにはこのイテレータがあります。`chunk_by_mut`（Rust 1.77 で安定化）は
両端から取り出せる `ChunkByMut` を返し、その実装は模範解答とほぼ同じです。構造体に入れるには、
述語に名前を付けられる型（関数ポインタなど）が必要です。

```rust
use std::slice::ChunkByMut;

pub struct RunsMut<'a, T> {
    inner: ChunkByMut<'a, T, fn(&T, &T) -> bool>,
}

pub fn runs_mut<T: PartialEq>(v: &mut [T]) -> RunsMut<'_, T> {
    let same: fn(&T, &T) -> bool = |a, b| a == b;
    RunsMut { inner: v.chunk_by_mut(same) }
}

impl<'a, T: PartialEq> Iterator for RunsMut<'a, T> {
    type Item = &'a mut [T];

    fn next(&mut self) -> Option<&'a mut [T]> {
        self.inner.next()
    }
}

impl<'a, T: PartialEq> DoubleEndedIterator for RunsMut<'a, T> {
    fn next_back(&mut self) -> Option<&'a mut [T]> {
        self.inner.next_back()
    }
}
```
