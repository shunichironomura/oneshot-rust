`&str` は UTF-8 のバイト列の中の**バイト**単位の位置でスライスします。1 文字は 1〜4 バイトです。
`&s[..max]` は、`max` が文字の途中に当たるとき
（`byte index 2 is not a char boundary; it is inside 'é'`）や `s` の末尾を超えるときにパニックします。
そのため、まずバイト数の上限を正しいバイト位置に直し、それからスライスする必要があります。

`str::floor_char_boundary`（Rust 1.91 で安定化）は、バイト位置を、それ以下で最も近い文字境界に
切り下げ、末尾を超える位置には `s.len()` を返します。`truncate_bytes` にはこれがそのまま使えます。
`ellipsize` では `s.chars().count()` ではなく `s.len()`（バイト数）と比べ、`'…'` のために 3 バイトを
空けておきます。`'…'` は 1 つの `char` ですが、`'…'.len_utf8()` は 3 です。

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
