配列を受け取って配列を返す関数を 3 つ実装してください。長さは const ジェネリックパラメータなので、
どの関数もどんな長さの配列にも使えます。

```rust
pub fn shout<const N: usize>(names: [String; N]) -> [String; N]
pub fn transpose<const R: usize, const C: usize>(m: [[i32; C]; R]) -> [[i32; R]; C]
pub fn zip_arrays<A, B, const N: usize>(a: [A; N], b: [B; N]) -> [(A, B); N]
```

- `shout` は、それぞれの名前を `str::to_uppercase` で大文字にし、末尾に `!` を付けます。
  結果は元の名前と同じ位置に置きます。`to_uppercase` は Unicode の規則に従うので、
  ASCII 以外の文字も変換され、名前が長くなることもあります（`"ß"` は `"SS"` になります）。
- `transpose` は行列 `m` の転置を返します。`m` は `R` 行で、各行に `C` 個の数が並んでいます。
  結果は `C` 行で、各行に `R` 個の数が並び、`result[c][r] == m[r][c]` となります。
- `zip_arrays` は同じ位置にある要素どうしを組にします。`result[i]` は `(a[i], b[i])` です。
  `A` と `B` はどんな型でもよく、`Copy` も `Clone` も実装していない型のこともあります。
- 3 つの関数はどれも、長さが 0 のとき（`N = 0`、`R = 0`、`C = 0`）にも動かなければなりません。
  たとえば `transpose::<0, 3>([])` は空の行を 3 つ返します。

### 例

- `shout(["ann".to_string(), "bob".to_string()])` は `["ANN!", "BOB!"]`（要素は `String`）を返します。
- `transpose([[1, 2, 3], [4, 5, 6]])` は `[[1, 4], [2, 5], [3, 6]]` を返します。
- `zip_arrays([1, 2], ["a", "b"])` は `[(1, "a"), (2, "b")]` を返します。
