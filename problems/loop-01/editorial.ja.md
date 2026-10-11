3 つの関数は、ブロックを途中で抜けるときの規則を 1 つずつ破っています。最初のコンパイルで 3 つとも報告されますが、
`find_cell` では、コンパイラの提案どおりに直すと間違ったコードになります。

1. **値を持って `break` できるのは `loop` とラベル付きブロックだけです。** `for` ループは `while` ループと同じく、
   イテレータが尽きると自然に終わることもあり、そのときは返す値がありません。そのため `for` ループの値は
   常に `()` で、`break 'search Some((r, c))` はエラー（E0571）になります。関数の本体がそのループで終わる
   こともエラー（E0308）です。テンプレートには、見つからなかったときに何を返すかがどこにも書かれていません。
   ラベル付きブロック `'search: { ... }` なら値を持てます。`break 'search value` は入れ子の `for` ループの
   中からでもブロックをその値で抜け、最後まで進んだときはブロックの最後の式（ここでは `None`）が値になります。
   コンパイラの提案をうのみにしないでください。`break 'search;` にして最後に `None` を足すとコンパイルは
   通りますが、常に `None` を返します。
2. **let-else の `else` は発散しなければなりません。** パターンにマッチしなかったときは `key` と `value` に
   値がないので、この文より後のコードを実行してはいけません。`else` ブロックは `return`、`break`、`continue`
   で抜けるか、パニックする必要があります（型が `!` でなければなりません）。`None` と書くだけでは
   ブロックの値になるだけで、関数から返るわけではありません。`return None;` と書きます。
3. **ラベルのない `break` がラベル付きブロックを抜けることはありません。** ラベルのない `break`
   （や `continue`）が指すのは、いちばん内側のループです。`break` が何を抜けるのかが常にはっきりするように、
   Rust は、ラベル付きブロックの中で、そのブロック内のループの外に書かれたラベルのない `break` を拒否します
   （E0695）。ブロックの周りにループがまったくなくても同じです。`break 'check "empty"` と書きます。
   `for` ループの中にある `break 'check "negative"` は、最初から正しいコードです。抜ける先はループではなく
   ブロックだからです。なお、ここにラベルのない `break` を書くとコンパイルは通りますが、抜けるのは
   `for` ループだけです。

`parse_setting` が返すキーは `line` を借用しています。ライフタイム省略規則により、返り値の `&str` は
ただ 1 つの参照引数と同じライフタイムを持ちます。`split_once` も `trim` も受け取った文字列のスライスを
返すので、`String` は要りません。

### 別解

ラベル付きブロックが関数の本体全体なら、`return` でも同じことができ、Rust のコードではこちらが一般的です。
`classify` は `if` / `else if` の連鎖でも読みやすく書けます。

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

pub fn classify(xs: &[i32]) -> &'static str {
    if xs.is_empty() {
        "empty"
    } else if xs.iter().any(|&x| x < 0) {
        "negative"
    } else if xs.contains(&0) {
        "zero"
    } else {
        "positive"
    }
}
```

`?` は `Option` にも使えます。値が `None` なら関数から `None` を返します。`parse_setting` の let-else は
これで置き換えられ、解釈する数の型は返り値の型から推論されます。

```rust
pub fn parse_setting(line: &str) -> Option<(&str, u32)> {
    let (key, value) = line.split_once('=')?;
    let key = key.trim();
    if key.is_empty() {
        return None;
    }
    Some((key, value.trim().parse().ok()?))
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
