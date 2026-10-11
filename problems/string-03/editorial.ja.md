`&str` は UTF-8 のバイト列の中の**バイト**単位の位置でスライスします。1 文字は 1〜4 バイトです。
そのため `&s[..n]` は文字ではなくバイトを数えますし、`n` が文字の途中に当たると
（`byte index 2 is not a char boundary; it is inside 'é'`）、あるいは `s` の末尾を超えるとパニックします。
どの関数でも、まず文字数やバイト数の上限を正しいバイト位置に直し、それからスライスする必要があります。

`char_indices` は各文字を、その文字が始まるバイト位置と一緒に返します。`first_chars` では、0 から数えて
`n` 番目の文字が始まる位置が、ちょうど先頭 `n` 文字の終わりになります。そこまで文字がなければ
`map_or(s.len(), ...)` で `s` 全体を取ります。`last_chars` は `rev()` で後ろからたどり、たどった `n` 文字のうち
最後の文字が末尾部分の始まりになります。`take(n).last()` なら `n = 0` も扱えます。何もたどらないので、結果は `s` の
末尾にある空のスライスになります。つい書きたくなる `rev().nth(n - 1)` は、このとき `0 - 1` が `usize` で
オーバーフローしてパニックします。

`str::floor_char_boundary`（Rust 1.91 で安定化）は、バイト位置を直前の文字境界まで切り下げ、末尾を超える位置には
`s.len()` を返します。`truncate_bytes` にはこれがそのまま使えます。`ellipsize` では `s.chars().count()` ではなく
`s.len()`（バイト数）と比べ、`'…'` のために 3 バイトを空けておきます。`'…'` は 1 つの `char` ですが、
`'…'.len_utf8()` は 3 です。

参照の引数が 1 つなので、ライフタイムの省略規則により、返す `&str` は `s` に結び付きます。`s` をスライスする
だけでよく、何もコピーしません。空の結果ならリテラルの `""` でも値は同じですが、指している場所が違います。
そのため、ここでは空の結果も `s` をスライスして作っています。

### 別解

`floor_char_boundary` を使わない場合は、`is_char_boundary` が真になるまで `max` から 1 ずつ戻ります。
位置 0 は必ず文字境界なので、ループは終わります。

```rust
pub fn truncate_bytes(s: &str, max: usize) -> &str {
    if max >= s.len() {
        return s;
    }
    let mut end = max;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}
```

`last_chars` は、先に文字数を数えて末尾 `n` 文字以外を読み飛ばす方法でも書けます。読み飛ばす部分がどこで終わるかは
`first_chars` で求められます。`n` が文字数より大きいとき、`saturating_sub` はアンダーフローせずに 0 を返します。

```rust
pub fn last_chars(s: &str, n: usize) -> &str {
    let skip = s.chars().count().saturating_sub(n);
    &s[first_chars(s, skip).len()..]
}
```

先頭（または末尾）の `n` 文字の `len_utf8` を足し合わせれば、先頭部分（または末尾部分）のバイト数が直接求まります。
`n = 0` なら合計は 0 なので、`last_chars` は `&s[s.len()..]` を返します。

```rust
pub fn first_chars(s: &str, n: usize) -> &str {
    let len: usize = s.chars().take(n).map(char::len_utf8).sum();
    &s[..len]
}

pub fn last_chars(s: &str, n: usize) -> &str {
    let len: usize = s.chars().rev().take(n).map(char::len_utf8).sum();
    &s[s.len() - len..]
}
```
