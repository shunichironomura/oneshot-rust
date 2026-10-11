`grid[(x, y)]` は `*grid.index((x, y))` の糖衣構文です。代入したり可変で借用したりするときは
`*grid.index_mut((x, y))` になります。そのため `index` はセルへの**参照**（`&Self::Output`、ここでは `&T`）を返し、
`IndexMut` は前提となる `Index` の `Output` 型をそのまま使います。メソッドが参照を返すので、
`g[(1, 0)].push_str("hi")` のように `Copy` でないセルにも添字でアクセスできます。

範囲の確認は座標ごとに行う必要があります。`self.cells[y * self.width + x]` がパニックするのは、
1 次元にした添字が `Vec` の末尾を超えたときだけです。3 × 2 のグリッドの `(3, 0)` は、何も言わずに `(0, 1)` になってしまいます。
`x < width && y < height` は添字を計算する**前に**確かめてください。`usize::MAX` のような大きな座標では
`y * width + x` がオーバーフローし、`None` を返す代わりにデバッグビルド（と Miri）でパニックします。
模範解答では、非公開のヘルパー `offset` でこの確認を 1 か所にまとめ、`get`、`index`、`index_mut` から使っています。

`row` にも専用の確認が必要です。`width == 0` だと `&self.cells[y * 0..y * 0 + 0]` はどんな `y` でも空のスライスになり、
パニックしません。

`row_sum` では、`start_bound()` と `end_bound()` が返すのは `Bound<&usize>`、つまり**参照**を持つ境界です。
パターン `Bound::Included(&s)` で数を取り出します。含まない始端と含む終端には `+ 1` が必要ですが、
`usize::MAX + 1` はオーバーフローします。`checked_add(1)?` を使えばこれが `None` になり、
そのような範囲はどのグリッドよりも先まで届くので、それが正しい答えです。あとは `slice::get(start..end)` が、
`end > len` のときも `start > end` のときも `None` を返すので、問題文の規則にちょうど一致します。

最後に、合計は `i64` で求めます。`iter().sum::<i32>()` だと `i32::MAX` が 3 つでオーバーフローするので、
先に各値を `i64::from` で変換します（`as i64` でも動きますが、`from` なら値が失われないことがはっきりします）。

### 別解

スライスは `Bound<usize>` の組で直接添字アクセスでき、その組を渡した `slice::get` はオーバーフローの確認も自分で行います。
`Bound::cloned` で `Bound<&usize>` を `Bound<usize>` に変換できます。

```rust
impl Grid<i32> {
    pub fn row_sum<R: RangeBounds<usize>>(&self, y: usize, cols: R) -> Option<i64> {
        if y >= self.height {
            return None;
        }
        let range = (cols.start_bound().cloned(), cols.end_bound().cloned());
        let cells = self.row(y).get(range)?;
        Some(cells.iter().map(|&v| i64::from(v)).sum())
    }
}
```

添字アクセスは、`get` と非公開の `get_mut` の上に作り、`expect` でパニックさせることもできます。

```rust
impl<T> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            self.cells.get(y * self.width + x)
        } else {
            None
        }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            self.cells.get_mut(y * self.width + x)
        } else {
            None
        }
    }

    // `row` は模範解答と同じです。
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &T {
        self.get(x, y).expect("cell out of range")
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        self.get_mut(x, y).expect("cell out of range")
    }
}
```
