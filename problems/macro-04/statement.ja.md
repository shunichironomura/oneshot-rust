次のコードはコンパイルが通りません。直してください。

パターンを受け取る宣言的マクロが 1 つと、それを使う関数が 2 つあります。マクロの後には、
ほかのモジュールからパスで呼べるようにする `pub(crate) use` の行があります。

```rust
macro_rules! matches_any { /* ... */ }
pub(crate) use matches_any;

pub fn is_vowel(c: char) -> bool;
pub fn is_small(x: Option<u8>) -> bool;
```

- `matches_any!(value, PATTERN)` は、`value` が `PATTERN` にマッチすれば `true`、しなければ `false` です。
  `PATTERN` には `match` のアームに書けるパターンなら何でも書けます。最上位の or パターン（`1 | 4..=6`）や、
  ほかのパターンの中の or パターン（`Some(1 | 2)`）も書けます。
- `value` には任意の式を書け、ちょうど 1 回だけ評価されます。
- `is_vowel(c)` は、`c` が ASCII の母音 `a`、`e`、`i`、`o`、`u` のどれか（小文字でも大文字でもよい）なら
  `true` です。
- `is_small(x)` は、`x` が `None` か、`n` が 0 から 9 の `Some(n)` なら `true` です。

テストは別のモジュールから、`crate::user::matches_any!(c, 'a' | 'b')` のようにパスでマクロを呼び出します。
名前、マクロの呼び出し方、関数のシグネチャは変えず、`pub(crate) use` の行は残してください。

### 例

```rust
crate::user::matches_any!(5, 1 | 4..=6)                // true
crate::user::matches_any!(Some(7), Some(1..=3) | None) // false
is_vowel('E')                                          // true
is_small(Some(12))                                     // false
```
