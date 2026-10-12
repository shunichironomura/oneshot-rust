`grid[(x, y)]` は `*grid.index((x, y))` の糖衣構文です。代入したり可変で借用したりするときは
`*grid.index_mut((x, y))` になります。そのため `index` はセルへの**参照**（`&Self::Output`、ここでは `&T`）を返し、
`IndexMut` は前提となる `Index` の `Output` 型をそのまま使います。メソッドが参照を返すので、
`g[(1, 0)].push_str("hi")` のように `Copy` でないセルにも添字でアクセスできます。

範囲の確認は座標ごとに行う必要があります。`self.cells[y * self.width + x]` がパニックするのは、
1 次元にした添字が `Vec` の末尾を超えたときだけです。3 × 2 のグリッドの `(3, 0)` は、何も言わずに `(0, 1)` になってしまいます。
`x < width && y < height` は添字を計算する**前に**確かめてください。`usize::MAX` のような大きな座標では
`y * width + x` がオーバーフローし、`None` を返す代わりにデバッグビルド（と Miri）でパニックします。
模範解答では、非公開のヘルパー `offset` でこの確認を 1 か所にまとめ、`get`、`index`、`index_mut` から使っています。

### 別解

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

    // `row` はテンプレートのままです。
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
