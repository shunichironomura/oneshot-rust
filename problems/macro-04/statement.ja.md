次のコードはコンパイルが通りません。直してください。

パターンを受け取る宣言的マクロが 2 つと、それを使う関数が 3 つあります。各マクロの後には、
ほかのモジュールからパスで呼べるようにする `pub(crate) use` の行があります。

```rust
macro_rules! matches_any { /* ... */ }
pub(crate) use matches_any;

macro_rules! index_of { /* ... */ }
pub(crate) use index_of;

pub fn is_vowel(c: char) -> bool;
pub fn is_small(x: Option<u8>) -> bool;
pub fn char_class(c: char) -> Option<usize>;
```

- `matches_any!(value, PATTERN)` は、`value` が `PATTERN` にマッチすれば `true`、しなければ `false` です。
  `PATTERN` には `match` のアームに書けるパターンなら何でも書けます。最上位の or パターン（`1 | 4..=6`）や、
  ほかのパターンの中の or パターン（`Some(1 | 2)`）も書けます。
- `index_of!(value; P0, P1, ...)` は、このようなパターンをカンマで区切って 1 つ以上受け取ります。
  末尾のカンマがあってもかまいません。結果は `Option<usize>` で、`value` がマッチする最初のパターン `Pi`
  （0 から数える）について `Some(i)`、どのパターンにもマッチしなければ `None` です。
- どちらのマクロでも、`value` には任意の式を書け、ちょうど 1 回だけ評価されます。
- `is_vowel(c)` は、`c` が ASCII の母音 `a`、`e`、`i`、`o`、`u` のどれか（小文字でも大文字でもよい）なら
  `true` です。
- `is_small(x)` は、`x` が `None` か、`n` が 0 から 9 の `Some(n)` なら `true` です。
- `char_class(c)` は、ASCII の英字なら `Some(0)`、ASCII の数字なら `Some(1)`、スペース（`' '`）、
  タブ（`'\t'`）、改行（`'\n'`）なら `Some(2)`、それ以外の文字なら `None` です。

テストは別のモジュールから、`crate::user::index_of!(c; 'a', 'b')` のようにパスでマクロを呼び出します。
名前、マクロの呼び出し方、関数のシグネチャは変えず、`pub(crate) use` の行は残してください。

### 例

```rust
crate::user::matches_any!(5, 1 | 4..=6)                // true
crate::user::matches_any!(Some(7), Some(1..=3) | None) // false
crate::user::index_of!(7; 0..=4, 5..=9, _)             // Some(1)
is_vowel('E')                                          // true
is_small(Some(12))                                     // false
char_class('7')                                        // Some(1)
```
