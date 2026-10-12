`Grid<T>` は、`width` × `height` の長方形に並んだセルを 1 つの `Vec<T>` に行ごとに格納します。
グリッドは `new`、`get`、`row`、`grid[(x, y)]` による添字アクセスと一緒にエディタに入っています。
`i32` のグリッドで、どんな形の範囲でも受け付けて行の合計を求めるメソッドを実装してください。

```rust
impl Grid<i32> {
    pub fn row_sum<R: RangeBounds<usize>>(&self, y: usize, cols: R) -> Option<i64>;
}
```

- セル `(x, y)` は列 `x`（`0..width`）、行 `y`（`0..height`）にあります。`row(y)` は行 `y` を、
  `x` の順に並んだ `width` 個のセルのスライスとして返し、`y >= height` ならパニックします。
- `row_sum(y, cols)` は、行 `y` のうち `cols` で選んだ列のセルを合計します。
  `cols` には `..`、`a..`、`..b`、`..=b`、`a..b`、`a..=b` のどの形の範囲も渡せるほか、
  `(Bound::Excluded(a), Bound::Included(b))` のような `std::ops::Bound` の組も渡せます。
  選ぶ列は `start..end` で、`start` と `end` は次のとおりです。
  - `start` は、始端を含むなら `a`、含まないなら `a + 1`、始端の指定がなければ `0` です。
  - `end` は、終端を含むなら `b + 1`、含まないなら `b`、終端の指定がなければ `width` です。
- `row_sum` は、`y >= height`、`end > width`、`start > end` のいずれかなら `None` を返します。
  比較は正確な値で行います。たとえば `..=usize::MAX` は列 `usize::MAX` の後ろで終わるので、
  どのグリッドでも `None` になります。
- そうでなければ、`row_sum` は選んだセルの正確な合計を返します（合計は `i32` に収まらないこともあります）。
  `start == end` なら何も選ばれず、結果は `Some(0)` です。幅 3 のグリッドでは、`3..3` は `Some(0)` に
  なりますが、`4..4` は `end > width` なので `None` になります。

### 例

```rust
let mut g = Grid::new(3, 2, 0);
g[(2, 1)] = 5;
g.row_sum(1, ..)    // Some(5)
g.row_sum(1, ..2)   // Some(0)
```

行が `[10, 20, 30, 40]` の 4 × 1 のグリッドでは次のようになります。

```rust
g.row_sum(0, 1..3)                                         // Some(50)
g.row_sum(0, ..=2)                                         // Some(60)
g.row_sum(0, 2..)                                          // Some(70)
g.row_sum(0, (Bound::Excluded(0), Bound::Included(2)))     // Some(50)
g.row_sum(0, 3..5)                                         // None（幅を超えている）
```
