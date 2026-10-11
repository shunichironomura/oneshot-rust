`String` は UTF-8 なので、1 文字が複数バイトになることがあります（`'é'` は 2 バイト）。`&word[..1]` のように
切り出すと、`"élan"` ではバイト位置 1 が文字の境界ではないのでパニックします。`chars()` で 1 文字ずつ見れば、
バイト位置を気にする必要はありません。

`char::to_uppercase` は `char` ではなくイテレータを返します。大文字にすると複数の文字になるものがあるからです
（`'ß'` → `"SS"`）。`String::extend` でそれらをすべて追加します。`to_ascii_uppercase` では `'é'` や `'ñ'` が
変わりません。

次の文字が単語の先頭かどうかをフラグで覚えておくと、空白はそのままコピーできます。`s.split_whitespace()` で
分けて `join(" ")` でつなぐと、タブや改行、連続した空白が失われます。

### 別解

`split_inclusive` で空白文字の直後ごとに区切ると、各部分は単語の先頭の文字か空白文字で始まります
（空白文字は大文字にしても変わりません）。

```rust
pub fn capitalize_words(s: &str) -> String {
    s.split_inclusive(char::is_whitespace)
        .map(|piece| {
            let mut chars = piece.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect()
}
```
