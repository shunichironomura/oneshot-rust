次のコードはコンパイルが通りません。直してください。

スライスについての質問に答える小さな関数が 3 つあります。

```rust
pub fn first_negative(v: &[i32]) -> Option<i32>
pub fn position_of_max(v: &[i32]) -> Option<usize>
pub fn count_long(words: &[&str], min: usize) -> usize
```

- `first_negative` は、`0` より小さい最初の要素を返します。そのような要素がなければ `None` を返します。
- `position_of_max` は、最大の要素の添字を返します。最大値が複数回現れるときは、最初に現れた位置の添字を返します。
  空のスライスなら `None` を返します。
- `count_long` は、長さが `min` バイト以上の単語がいくつあるかを返します。長さは `len()` が返すバイト数で、
  文字数ではありません（`"é"` の長さは 2 バイトです）。

シグネチャは変えないでください。

### 例

- `first_negative(&[3, -1, -5])` は `Some(-1)` を返します。
- `position_of_max(&[1, 9, 9])` は `Some(1)` を返します。
- `count_long(&["a", "abc", "de"], 2)` は `2` を返します。
