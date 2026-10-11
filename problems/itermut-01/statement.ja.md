可変スライスの**連なり**（等しい要素が隣り合って並んだまとまり）を順に返すイテレータを
書いてください。各連なりは可変の部分スライスとして渡すので、呼び出し側はそれを通して
書き換えられます。連なりは前からも後ろからも取り出せます。

```rust
pub struct RunsMut<'a, T> {
    rest: &'a mut [T],
}

pub fn runs_mut<T: PartialEq>(v: &mut [T]) -> RunsMut<'_, T>

impl<'a, T: PartialEq> Iterator for RunsMut<'a, T> {
    type Item = &'a mut [T];
    fn next(&mut self) -> Option<&'a mut [T]>;
}

impl<'a, T: PartialEq> DoubleEndedIterator for RunsMut<'a, T> {
    fn next_back(&mut self) -> Option<&'a mut [T]>;
}
```

`RunsMut` はエディタに入っています。フィールドは非公開なので、変更してもかまいません。

- `v` を、それぞれができるだけ長くなるように連なりに分けます。直前の要素と等しくない要素の
  ところで、新しい連なりが始まります。どの連なりにも 1 つ以上の要素があり、空の `v` には
  連なりがありません。等しい値でも、隣り合っていなければ別の連なりです。
- 要素は `==` で比べます。`T` に求めるのは `PartialEq` だけなので、値が自分自身と等しいとは
  限りません。`f64::NAN == f64::NAN` は `false` なので、NaN はどれも、それだけで 1 つの連なりに
  なります。
- `next` は連なりを前から（左から右へ）、`next_back` は後ろから（右から左へ）返します。
  2 つは混ぜて呼べます。どちらの端から取り出しても、各連なりはちょうど 1 回だけ返されます。
  すべての連なりを返し終えたら、どちらも `None` を返し、その後も `None` を返し続けます。
- 各連なりは、`v` の中でその連なりが占める部分スライスとして返します。呼び出し側は、
  返された連なりをすべて持ったまま、イテレータがなくなった後でも、それらを同時に読み書き
  できます。書き込みは `v` に反映されます。
- パニックしてはいけません。`unsafe` なコードは使えません。

### 例

```rust
let mut v = [1, 1, 2, 3, 3, 3];
let runs: Vec<&mut [i32]> = runs_mut(&mut v).collect(); // [[1, 1], [2], [3, 3, 3]]
for run in runs {
    let len = run.len() as i32;
    run.fill(len);
}
// v は [2, 2, 1, 3, 3, 3] になる

let mut v = [1, 1, 2, 3, 3, 3];
runs_mut(&mut v).rev().collect::<Vec<_>>() // [[3, 3, 3], [2], [1, 1]]

let mut it = runs_mut(&mut v);
it.next()      // Some([1, 1])
it.next_back() // Some([3, 3, 3])
it.next()      // Some([2])
it.next()      // None
it.next_back() // None
```
