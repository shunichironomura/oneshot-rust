単語のリストに繰り返しがあるかを調べてください。2 回目に出てくる最初の単語と、
どの単語も 1 回しか出てこないかどうかを求めます。

```rust
pub fn first_repeat<'a>(words: &[&'a str]) -> Option<&'a str>
pub fn all_unique(words: &[&str]) -> bool
```

- 単語はそのままの形で比べます。`"A"` と `"a"` は別の単語です。
- `first_repeat` は `words` を先頭から読んでいき、それより前にすでに出てきた単語に初めて出会ったら、
  その単語を返します。2 回以上出てくる単語がなければ `None` を返します。
- `all_unique` は、`words` に 2 回以上出てくる単語がなければ `true` を、あれば `false` を返します。
  空のリストは `true` です。

### 例

```rust
first_repeat(&["a", "b", "c", "b", "a"])
// Some("b")（2 つ目の "b" は 2 つ目の "a" より前にある）
first_repeat(&["a", "b", "c"])  // None

all_unique(&["a", "b", "c"])    // true
all_unique(&["a", "b", "a"])    // false
```
