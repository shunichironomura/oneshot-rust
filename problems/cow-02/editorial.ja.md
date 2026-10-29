まず調べて、本当に何かが変わるときだけコピーします。

`Cow<'_, str>` は `Borrowed(&str)` か `Owned(String)` のどちらかで、`'_` によって借用した結果が `s` に
結びつきます。`s.trim_ascii()` はすでに `s` のスライスなので、内側を変える必要がなければ、何も確保せずに
そのまま `Cow::Borrowed` で返せます。`String` を作るのは、2 文字以上の連なりか、普通の空白でない連なりが
あるときだけです。`str::trim` と `split_whitespace` は Unicode の定義の空白を使い、ASCII より
広い範囲を空白として扱うので注意してください。ASCII 版は `trim_ascii` と `split_ascii_whitespace` です。

### 別解

正規化した文字列を作ってから、トリムしたスライスと比べる方法です。短く書けますが、結果を借用で返すときでも
確保が発生します。

```rust
pub fn normalize_ws(s: &str) -> Cow<'_, str> {
    let trimmed = s.trim_ascii();
    let joined = trimmed.split_ascii_whitespace().collect::<Vec<_>>().join(" ");
    if joined == trimmed {
        Cow::Borrowed(trimmed)
    } else {
        Cow::Owned(joined)
    }
}
```
