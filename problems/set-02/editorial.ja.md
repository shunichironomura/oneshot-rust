`HashSet::insert` は 2 つのことを一度にします。値を追加し、その値が新しければ `true`、すでに集合に
あれば `false` を返します。そのため `insert` の前に `contains` を呼ぶ必要はありません。

`first_repeat` では、`insert` が初めて `false` を返した単語が答えです。`for &word in words` は各
`&&'a str` を分解して `&'a str` を取り出すので、その単語をそのまま返せます。この単語が借用しているのは
元の文字列で、`words` や集合ではありません。

`all_unique` では、すべての `insert` が `true` を返す必要があります。`Iterator::all` は最初の `false` で
止まります。`all` は `FnMut` のクロージャを受け取るので、クロージャの中で `seen` を変更できます。

### 別解

`first_repeat` は `Iterator::find` を使うと 1 つの式で書けます。

```rust
pub fn first_repeat<'a>(words: &[&'a str]) -> Option<&'a str> {
    let mut seen = HashSet::new();
    words.iter().copied().find(|&word| !seen.insert(word))
}
```

`first_repeat` があれば、`all_unique` はその結果が `None` かどうかを見るだけです。

```rust
pub fn all_unique(words: &[&str]) -> bool {
    first_repeat(words).is_none()
}
```
