**値を持って `break` できるのは `loop` とラベル付きブロックだけです。** `for` ループは `while` ループと同じく、
イテレータが尽きると自然に終わることもあり、そのときは返す値がありません。そのため `for` ループの値は
常に `()` で、`break 'search Some((r, c))` はエラー（E0571）になります。関数の本体がそのループで終わる
こともエラー（E0308）です。テンプレートには、見つからなかったときに何を返すかがどこにも書かれていません。

ラベル付きブロック `'search: { ... }` なら値を持てます。`break 'search value` は入れ子の `for` ループの
中からでもブロックをその値で抜け、最後まで進んだときはブロックの最後の式（ここでは `None`）が値になります。

コンパイラの提案をうのみにしないでください。`break 'search;` にして最後に `None` を足すとコンパイルは
通りますが、常に `None` を返します。

### 別解

ラベル付きブロックが関数の本体全体なら、`return` でも同じことができ、Rust のコードではこちらが一般的です。

```rust
pub fn find_cell(grid: &[Vec<i32>], target: i32) -> Option<(usize, usize)> {
    for (r, row) in grid.iter().enumerate() {
        for (c, &value) in row.iter().enumerate() {
            if value == target {
                return Some((r, c));
            }
        }
    }
    None
}
```

値を持たない `break` なら、`for` ループのラベルを指定できます。テンプレートのループを残すなら、答えを変数に
入れてから `break 'search;` で両方のループを抜けます。ラベルのない `break` では内側のループしか抜けず、
後の行で見つかったマスが答えを上書きしてしまいます。

```rust
pub fn find_cell(grid: &[Vec<i32>], target: i32) -> Option<(usize, usize)> {
    let mut found = None;
    'search: for (r, row) in grid.iter().enumerate() {
        for (c, &value) in row.iter().enumerate() {
            if value == target {
                found = Some((r, c));
                break 'search;
            }
        }
    }
    found
}
```

イテレータを使うなら、`position` で 1 つの行の中の列を探し、`find_map` で見つかった最初の行で止めます。

```rust
pub fn find_cell(grid: &[Vec<i32>], target: i32) -> Option<(usize, usize)> {
    grid.iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|&value| value == target).map(|c| (r, c)))
}
```
