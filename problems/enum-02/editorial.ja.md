`&[Cell]` を反復すると `&Cell` が得られます。参照を `Cell::Int(n)` のようなパターンにマッチさせると、
フィールドも参照として束縛されます（マッチエルゴノミクス）。`n` は `&i64`、`x` は `&f64`、`s` は
`&String` です。この問題のコンパイルエラーの多くは、これを忘れることから起こります。

- `n as f64` は「`&i64` を `f64` にキャストできない」というエラーになります。`*n as f64` と書きます。
  同じように、値を返す `match` で `Cell::Float(x) => x` と書くと、`f64` が必要な所に `&f64` が来ます。
- `texts` で `Some(s)` とすると `&String` のイテレータになり、`collect` でそこから `Vec<&str>` は
  作れません。参照外し型強制（deref coercion）で `&String` が `&str` になるのは、変換先の型がすでに
  わかっている所だけで、`collect` の中では起きません。`s.as_str()` で明示的に変換します。

`texts` が返すスライスは、`row` の中の `String` を指しています。参照の引数が 1 つだけなので、
ライフタイムの省略規則によって、結果はその引数に結び付けられます。
`fn texts<'a>(row: &'a [Cell]) -> Vec<&'a str>` と書いたのと同じです。何もコピーせずに済み、
結果は `row` が生きている間使えます。

`parse_cell` では、調べる順番が重要です。`i64` のパーサーが受け付ける文字列は、すべて `f64` のパーサーも
受け付けます。そのため `f64` を先に試すと、`3` は `Float(3.0)` に、`-0` は `Float(-0.0)` になってしまいます。
標準のパーサーは符号、指数、`.5`、`5.`、`inf` をすでに受け付け、`1_000` や `0x10` は受け付けないので、
自分でチェックを書く必要はありません。`split(',')` は必ず 1 つ以上の断片を返すので、`""` は `[Empty]` に
なります。`Cell::Text` は `String` を所有するので、前後の空白を除いた `&str` には `to_string()` が必要です。

`numeric_sum` では、整数を 1 つずつ `f64` に変換してから足します。先に `Int` を `i64` のまま合計すると、
`i64::MAX` に近い値でオーバーフローします（デバッグビルドではパニック）。

### 別解

`cell` ではなく `*cell` でマッチすると、フィールドは値として束縛されます。`i64` と `f64` は `Copy` で、
`_` のアームは何も束縛しないので、これで動きます。`Cell::Text(s)` のようなアームを書くと、借用している
行から `String` をムーブしようとすることになり、コンパイルできません。

```rust
pub fn numeric_sum(row: &[Cell]) -> f64 {
    row.iter()
        .map(|cell| match *cell {
            Cell::Int(n) => n as f64,
            Cell::Float(x) => x,
            _ => 0.0,
        })
        .sum()
}
```

`texts` はループでも書けます。`let mut out: Vec<&str>` と型を書いておけば、`push` の引数の型が
わかっているので `&String` が `&str` に変換され、`out.push(s)` でもコンパイルが通ります。型を書かないと、
最初の `push` で `out` が `Vec<&String>` になり、返り値の型と合いません。

```rust
pub fn texts(row: &[Cell]) -> Vec<&str> {
    let mut out = Vec::new();
    for cell in row {
        if let Cell::Text(s) = cell {
            out.push(s.as_str());
        }
    }
    out
}
```

タプルバリアントの名前は関数としても使えます（`Cell::Int: fn(i64) -> Cell` など）。そのため
`Result::map` に渡せます。こうすると、`parse` が作る型もターボフィッシュなしで決まります。

```rust
fn parse_cell(field: &str) -> Cell {
    if field.is_empty() {
        return Cell::Empty;
    }
    field
        .parse()
        .map(Cell::Int)
        .or_else(|_| field.parse().map(Cell::Float))
        .unwrap_or_else(|_| Cell::Text(field.to_string()))
}
```
