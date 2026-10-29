コピーは高くつくので、本当に何かを変えるときだけコピーするようにします。`s` の空白を整える関数
`normalize_ws` を実装してください。

```rust
use std::borrow::Cow;

pub fn normalize_ws(s: &str) -> Cow<'_, str>;
```

- 空白とは、`char::is_ascii_whitespace` が真になる文字のことです。
- 結果は、`s` の両端の空白を取り除き、空白でない 2 文字の間にある空白の連なりをそれぞれ 1 つの `' '` に
  置き換えた文字列です。
- 置き換える連なりが 1 つもない（内側の連なりがどれもちょうど 1 つの `' '` である）ときは、結果は `s` の一部
  そのものです。コピーせずに、`s` のスライスを `Cow::Borrowed` で返してください。それ以外のときは
  `Cow::Owned` を返します。

### 例

```rust
normalize_ws("a b")       // Borrowed("a b")
normalize_ws("  hi\n")    // Borrowed("hi")。入力のスライス
normalize_ws("a  b\tc ")  // Owned("a b c")
```
