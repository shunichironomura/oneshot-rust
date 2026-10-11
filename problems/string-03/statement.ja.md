狭い欄に表示する文字列や、サイズに上限のある欄に保存する文字列は、途中で切り詰めなければならないことがよくあります。
文字を途中で分断せずに文字列を短くする 4 つの関数を実装してください。

```rust
pub fn first_chars(s: &str, n: usize) -> &str
pub fn last_chars(s: &str, n: usize) -> &str
pub fn truncate_bytes(s: &str, max: usize) -> &str
pub fn ellipsize(s: &str, max: usize) -> String
```

ここでの「文字」は Rust の `char`（Unicode スカラー値）です。たとえば `"e\u{301}"`
（`e` の後に結合用のアクセント記号が続く）は 2 文字と数えます。バイト数は UTF-8 で表したときの長さ
（`str::len` が返す値）です。

- `first_chars` は `s` の先頭の `n` 文字を返します。`s` が `n` 文字以下なら `s` 全体を返します。
- `last_chars` は `s` の末尾の `n` 文字を返します。`s` が `n` 文字以下なら `s` 全体を返します。
- `truncate_bytes` は、`s` の先頭部分のうち、`max` バイト以下で文字を分断しない最長のものを返します。
- `ellipsize` は、`s` が `max` バイト以下なら `s` のコピーを返します。そうでなければ、`truncate_bytes(s, max - 3)` の
  後ろに `'…'`（U+2026。UTF-8 で 3 バイト）を付けたものを返します。したがって、結果が `max` バイトを超えることは
  ありません。`max` は 3 以上です。
- `&str` を返す 3 つの関数は、新しい文字列や文字列リテラルではなく、`s` 自身の一部を返します。`first_chars` と
  `truncate_bytes` は `s` の先頭から始まるスライスを、`last_chars` は `s` の末尾で終わるスライスを返します。
  結果が空のときも同じです。テストでは、返したスライスがどこを指しているかも確認します。
- `n` と `max` は、0 や `s` の長さより大きい値も含めて、どんな値も取り得ます（`max` が 3 以上でなければならないのは
  `ellipsize` だけです）。

### 例

| 呼び出し | 結果 |
| --- | --- |
| `first_chars("héllo", 2)` | `"hé"` |
| `last_chars("日本語です", 2)` | `"です"` |
| `truncate_bytes("héllo", 2)` | `"h"`（`é` は 2 バイトなので収まらない） |
| `truncate_bytes("héllo", 3)` | `"hé"` |
| `ellipsize("こんにちは", 10)` | `"こん…"`（6 + 3 = 9 バイト） |
| `ellipsize("hello!", 5)` | `"he…"` |
| `ellipsize("hello", 5)` | `"hello"` |
