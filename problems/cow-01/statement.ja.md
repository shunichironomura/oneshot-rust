コピーは高くつくので、本当に何かを変えるときだけコピーするようにします。`v` の値を `max` 以下に
抑える関数 `clamp_all` を実装してください。

```rust
use std::borrow::Cow;

pub fn clamp_all(v: &mut Cow<'_, [i32]>, max: i32);
```

- `max` より大きい要素はすべて `max` になります。それ以外の要素はそのままです。
- `max` より大きい要素が 1 つもなければ、`v` は何も変わりません。`Cow::Borrowed` は `Borrowed` のままで、
  何もコピーしません。
- そうでなければ、`Cow::Borrowed` は値を抑えた `Cow::Owned` になります。借用していたスライス自体は
  変わりません。
- `Cow::Owned` は `Owned` のままで、その `Vec` をその場で書き換えます（新しい `Vec` は作りません）。

### 例

```rust
let data = [1, 5, 3];
let mut v = Cow::Borrowed(&data[..]);
clamp_all(&mut v, 9);     // Borrowed([1, 5, 3]) のまま
clamp_all(&mut v, 4);     // Owned([1, 4, 3]) になる。data は変わらない
```
