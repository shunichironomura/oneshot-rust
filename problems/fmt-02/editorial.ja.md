どの関数も、書式指定のミニ言語 `{arg:[fill][align][#][0][width][.precision][type]}` で書けます。
模範解答の書式指定では、各部分がそれぞれ 1 つの役割を持っています。

- `{name:<10.10}`：幅 `10` は**最小**の文字数で、足りない分を空白で埋めます。文字列に付けた精度 `.10` は
  **最大**の文字数なので、長い名前は切り詰められます。精度がないと、`Chocolate bar` が行の残りを
  右に押し出してしまいます。
- `{qty:>3}` と `{price:>8.2}`：浮動小数点数に付けた精度は小数点以下の桁数で、値は丸められます
  （`19.999` は `20.00` になります）。幅で何かが切られることはないので、`1000` や `100000.00` は
  そのまま全体が出力されます。数値の既定は右寄せ、文字列の既定は左寄せなので、`{qty:3}` や
  `{price:8.2}` でも動きます。
- `{r:02X}`：`0` フラグは空白の代わりに 0 で埋め、`2` は幅、`X` は大文字の 16 進数です。`{r:X}` では
  `3` が 1 文字の `3` になって色が短くなり、`{r:2X}` では空白で埋められます。
- `{n:#010b}`：`#` は `0b` を付けます。幅にはこの `0b` も含まれるので、8 桁にするには幅 10 が必要です。
  よく間違えるのが `{n:#08b}` で、こちらは `0b000101` と 6 桁しか出力されません。
- `{cell:^width$}`：実行時にしかわからない幅は `name$` と書き、`name` という名前の引数
  （またはスコープ内の変数）から読み取ります。`{cell:^width}` はコンパイルが通りません。`width` が
  書式トレイトの名前として読まれるからです。中央寄せは、もともと余った空白を右側に置きます。

`"|"` から始めてセルごとに `cell|` を足していけば、空のスライスでも正しい行になります。
中央寄せしたセルを `parts` に集めて `format!("|{}|", parts.join("|"))` と書く方法では、
その場合に `||` になってしまいます。

### 別解

`0b` を自分で書く方法です。この場合、幅は数字の桁だけを数えます。

```rust
pub fn bits(n: u8) -> String {
    format!("0b{n:08b}")
}
```

`1$` は、インデックス 1 の位置引数から幅を読み取ります。また、`write!` を使うと、セルごとに一時的な
`String` を作らずに直接 `String` に書き込めます。`String` への書き込みは失敗しないので、
`fmt::Result` は `unwrap` して構いません。

```rust
use std::fmt::Write;

pub fn table_row(cells: &[&str], width: usize) -> String {
    let mut row = String::from("|");
    for cell in cells {
        write!(row, "{:^1$}|", cell, width).unwrap();
    }
    row
}
```

自分で中央寄せすると、`^` が何をしているかがわかります。`saturating_sub` は、`width` より長いセルで
アンダーフローしないようにするためです。`padding / 2` は切り捨てなので、余った空白は右側に置かれます。

```rust
pub fn table_row(cells: &[&str], width: usize) -> String {
    let mut row = String::from("|");
    for cell in cells {
        let padding = width.saturating_sub(cell.len());
        let left = padding / 2;
        row.push_str(&" ".repeat(left));
        row.push_str(cell);
        row.push_str(&" ".repeat(padding - left));
        row.push('|');
    }
    row
}
```
