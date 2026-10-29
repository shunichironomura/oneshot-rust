`str` は整数で添字アクセスできません（The Book の 8 章）。中身は UTF-8 のバイト列で、1 文字が 1〜4 バイトを
占めるので、「0 番目」がバイトなのか文字なのかが決まらず、Rust はどちらかを明示させます。
`word.chars().next()` は最初の `char` を `Option` で返します。`None` になるのは空文字列のときだけで、
`split_whitespace` が空の単語を返すことはありません。

コンパイルは通るけれど間違っている直し方も 2 つあります。

- `word.as_bytes()[0] as char` は `é` の最初のバイト（`0xC3`）を取り出して文字 `Ã` にしてしまいます。
- `&word[..1]` はパニックします。`str` の範囲はバイト単位で、バイト位置 1 は `é` の途中にあり、
  文字の境界ではないからです。

### 別解

`String` は `FromIterator<String>` を実装しているので、頭文字はイテレータから `collect` でも作れます。

```rust
pub fn initials(full_name: &str) -> String {
    full_name
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .map(|first| format!("{first}."))
        .collect()
}
```
