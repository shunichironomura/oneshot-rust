`Grid<T>` は、`width` × `height` の長方形に並んだセルを 1 つの `Vec<T>` に行ごとに格納します
（エディタに入っています）。コンストラクタ、`get`、`grid[(x, y)]` による添字アクセスを実装してください。

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
    pub fn row(&self, y: usize) -> &[T]; // 実装済み
}

impl<T> Index<(usize, usize)> for Grid<T> { type Output = T; /* ... */ }
impl<T> IndexMut<(usize, usize)> for Grid<T> { /* ... */ }
```

- セルは `(x, y)` で指定します。`x` は列で `0..width`、`y` は行で `0..height` の範囲です。
  セル `(x, y)` は `cells` の添字 `y * width + x` に格納されます。
- `new` は、すべてのセルが `fill` のグリッドを作ります。`width` や `height` は `0` のこともあります。
- `get(x, y)` はそのセルを返します。`x >= width` または `y >= height` なら `None` を返します。
  `usize::MAX` を含め、どんな `x` と `y` に対してもこのとおりです。
- `grid[(x, y)]` でセルを読み、`grid[(x, y)] = v`（や `+=` など）でセルを書き換えます。
  どちらも、`get(x, y)` が `None` を返す場合はパニックします。範囲は座標ごとに調べます。
  たとえば 3 × 2 のグリッドでは、`0 * 3 + 3` は `cells` の添字としては有効ですが、`(3, 0)` はパニックします。
- `row(y)` は実装済みです。行 `y` を、`x` の順に並んだ `width` 個のセルのスライスとして返します。

### 例

```rust
let mut g = Grid::new(3, 2, 0);
g[(2, 1)] = 5;
g[(0, 0)] += 7;
g[(0, 0)]           // 7
g.get(2, 1)         // Some(&5)
g.get(3, 0)         // None
g.row(1)            // [0, 0, 5]
```
