`Grid<T>` は、`width` × `height` の長方形に並んだセルを 1 つの `Vec<T>` に行ごとに格納します
（エディタに入っています）。コンストラクタとアクセサ、`grid[(x, y)]` による添字アクセス、
`i32` のグリッドで行の合計を求めるメソッドを実装してください。

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, fill: T) -> Self;
}

impl<T> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T>;
    pub fn row(&self, y: usize) -> &[T];
}

impl<T> Index<(usize, usize)> for Grid<T> { type Output = T; /* ... */ }
impl<T> IndexMut<(usize, usize)> for Grid<T> { /* ... */ }

impl Grid<i32> {
    pub fn row_sum<R: RangeBounds<usize>>(&self, y: usize, cols: R) -> Option<i64>;
}
```

- セルは `(x, y)` で指定します。`x` は列で `0..width`、`y` は行で `0..height` の範囲です。
  セル `(x, y)` は `cells` の添字 `y * width + x` に格納されます。
- `new` は、すべてのセルが `fill` のグリッドを作ります。`width` や `height` は `0` のこともあります。
- `get(x, y)` はそのセルを返します。`x >= width` または `y >= height` なら `None` を返します。
  `usize::MAX` を含め、どんな `x` と `y` に対してもこのとおりです。
- `grid[(x, y)]` でセルを読み、`grid[(x, y)] = v`（や `+=` など）でセルを書き換えます。
  どちらも、`get(x, y)` が `None` を返す場合はパニックします。範囲は座標ごとに調べます。
  たとえば 3 × 2 のグリッドでは、`0 * 3 + 3` は `cells` の添字としては有効ですが、`(3, 0)` はパニックします。
- `row(y)` は行 `y` を、`x` の順に並んだ `width` 個のセルのスライスとして返します。
  `y >= height` ならパニックします。`width` が `0` のときも同じです。
- `row_sum(y, cols)` は、行 `y` のうち `cols` で選んだ列のセルを合計します。
  `cols` には `..`、`a..`、`..b`、`..=b`、`a..b`、`a..=b` のどの形の範囲も渡せるほか、
  `(Bound::Excluded(a), Bound::Included(b))` のような `std::ops::Bound` の組も渡せます。
  選ぶ列は `start..end` で、`start` と `end` は次のとおりです。
  - `start` は、始端を含むなら `a`、含まないなら `a + 1`、始端の指定がなければ `0` です。
  - `end` は、終端を含むなら `b + 1`、含まないなら `b`、終端の指定がなければ `width` です。
- `row_sum` は、`y >= height`、`end > width`、`start > end` のいずれかなら `None` を返します。
  比較は正確な値で行います。たとえば `..=usize::MAX` は列 `usize::MAX` の後ろで終わるので、
  どのグリッドでも `None` になります。そうでなければ、`row_sum` は選んだセルの正確な合計を返します
  （合計は `i32` に収まらないこともあります）。このとき `start == end` なら何も選ばれず、
  結果は `Some(0)` です。幅 3 のグリッドでは、`3..3` は `Some(0)` になりますが、
  `4..4` は `end > width` なので `None` になります。

### 例

```rust
let mut g = Grid::new(3, 2, 0);
g[(2, 1)] = 5;
g[(0, 0)] += 7;
g[(0, 0)]           // 7
g.get(2, 1)         // Some(&5)
g.row(1)            // [0, 0, 5]
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
