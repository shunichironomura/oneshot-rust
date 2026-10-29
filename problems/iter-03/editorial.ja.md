`position` は、呼び出したイテレータで一致した要素の前にいくつ要素があったかを返すので、
`rev()` の後では後ろから数えることになります。
`words.iter().rev().position(|w| w.chars().count() >= 5)` は、
`["tea", "coffee", "milk", "juice"]` に対して `Some(0)` を返します。逆順のイテレータでは
`"juice"` が最初の要素だからです。`rposition` も後ろから探しますが、返す位置は先頭から
数えたものです。`rposition` には、長さがわかっていて後ろからもたどれるイテレータ
（`ExactSizeIterator` かつ `DoubleEndedIterator`）が必要で、`slice::Iter` はそれにあたります。
文字数は `chars().count()` で数えます。`len()` はバイト数で、`"日本"` は 6 バイトですが
2 文字です。

### 別解

`rev` の前に `enumerate` を呼ぶと、先頭から数えた位置が各要素に付き、逆順にしてもそのまま
残ります。（`rev().enumerate()` だと、また後ろから番号が付きます。）

```rust
pub fn last_long(words: &[&str], min: usize) -> Option<usize> {
    words
        .iter()
        .enumerate()
        .rev()
        .find(|(_, w)| w.chars().count() >= min)
        .map(|(i, _)| i)
}
```
