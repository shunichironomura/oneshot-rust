次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

3 つの関数は、どれも `impl Trait` を返します。列挙型 `Keep` はすでにエディタに入っています。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keep {
    Evens,
    Odds,
    All,
}

pub fn select(v: &[i32], keep: Keep) -> impl Iterator<Item = i32> + '_
pub fn describe(n: i32) -> impl Display
pub fn op_for(symbol: char, scale: i32) -> impl Fn(i32, i32) -> i32
```

- `select(v, keep)` は、`v` の要素を元の順に返します。`Keep::Evens` なら偶数だけ、
  `Keep::Odds` なら奇数だけ、`Keep::All` ならすべての要素です。負の数も同じように扱い、
  `-4` と `0` は偶数、`-3` は奇数です。
- `describe(n)` を（`{}` や `to_string()` で）表示すると、`n` が負なら単語 `minus`、
  空白 1 つ、`n` の絶対値を並べたもの、それ以外なら数そのものになります。`i32::MIN` を含む、
  すべての `i32` で正しく動く必要があります。
- `op_for(symbol, scale)` は関数 `f` を返します。`f(a, b)` の値は次のとおりです。
  - `'+'` なら `(a + b) * scale`
  - `'-'` なら `(a - b) * scale`
  - `'*'` なら `a * b * scale`
  - それ以外の記号なら `a * scale`（`b` は使いません）

  ハーネスは、返された関数をそれぞれ何度か呼ぶことがあります。値は小さいので、
  オーバーフローは起きません。

`Keep` と、`select`・`describe` のシグネチャは変えないでください。`op_for` は、
結果を `Fn` のように共有参照越しに `f(a, b)` の形で呼べるかぎり、返り値の型だけは変えて構いません。

### 例

- `select(&[1, -3, 4], Keep::Odds)` は `1`、`-3` の順に返します。
- `describe(-5).to_string()` は `"minus 5"`、`describe(12).to_string()` は `"12"` です。
- `op_for('-', 2)(7, 3)` は `8` です。
