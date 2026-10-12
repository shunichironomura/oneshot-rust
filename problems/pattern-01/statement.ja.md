次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

3 つの関数は、どれもパターンを使って入力を分解します。

```rust
pub fn first_even(v: &[i32]) -> Option<i32>;
pub fn title_or_default(title: Option<String>) -> (bool, String);
pub fn filled_slots(slots: &[Option<u8>]) -> Vec<(usize, u8)>;
```

- `first_even` は `v` の要素のうち最初の偶数を返します。偶数がなければ `None` です。0 や負の数も対象で、
  `0` と `-8` は偶数、`-3` は奇数です。
- `title_or_default` は、タイトルが指定されたかどうかと、表示するタイトルを返します。`Some(t)` なら
  `(true, t)` で、`t` は空文字列であってもそのまま返します。`None` なら `(false, "Untitled")` です。
- `filled_slots` は、`slots` のうち値の入っている枠（`Some`）それぞれについて、位置と値の組を順に
  返します。位置は 0 から始まり、`None` の枠も数えます。`Some(0)` は `0` が入っている枠です。

シグネチャは変えないでください。

### 例

| 呼び出し | 結果 |
| --- | --- |
| `first_even(&[1, 4, 6])` | `Some(4)` |
| `title_or_default(Some("Notes".to_string()))` | `(true, "Notes")` |
| `title_or_default(None)` | `(false, "Untitled")` |
| `filled_slots(&[None, Some(4), Some(7)])` | `[(1, 4), (2, 7)]` |
