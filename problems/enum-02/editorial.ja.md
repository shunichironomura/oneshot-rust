`&[Cell]` を反復すると `&Cell` が得られます。参照を `Cell::Int(n)` のようなパターンにマッチさせると、
フィールドも参照として束縛されます（マッチエルゴノミクス）。`n` は `&i64`、`s` は `&String` です。
この問題のコンパイルエラーの多くは、これを忘れることから起こります。

- 値を返す `match` で `Cell::Int(n) => n` と書くと、`i64` が必要な所に `&i64` が来ます。`*n` と書きます。
  （`sum += n` はたまたまコンパイルが通ります。`i64` は `AddAssign<&i64>` も実装しているからです。）
- `texts` で `Some(s)` とすると `&String` のイテレータになり、`collect` でそこから `Vec<&str>` は
  作れません。参照外し型強制（deref coercion）で `&String` が `&str` になるのは、変換先の型がすでに
  わかっている所だけで、`collect` の中では起きません。`s.as_str()` で明示的に変換します。

`texts` が返すスライスは、`row` の中の `String` を指しています。参照の引数が 1 つだけなので、
ライフタイムの省略規則によって、結果はその引数に結び付けられます。
`fn texts<'a>(row: &'a [Cell]) -> Vec<&'a str>` と書いたのと同じです。何もコピーせずに済み、
結果は `row` が生きている間使えます。

`parse_cell` について。`split(',')` は必ず 1 つ以上の断片を返すので、`""` は `[Empty]` になります。
`Cell::Text` は `String` を所有するので、前後の空白を除いた `&str` には `to_string()` が必要です。

### 別解

`cell` ではなく `*cell` でマッチすると、フィールドは値として束縛されます。`i64` は `Copy` で、
`_` のアームは何も束縛しないので、これで動きます。`Cell::Text(s)` のようなアームを書くと、借用している
行から `String` をムーブしようとすることになり、コンパイルできません。

```rust
pub fn numeric_sum(row: &[Cell]) -> i64 {
    row.iter()
        .map(|cell| match *cell {
            Cell::Int(n) => n,
            _ => 0,
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
        .unwrap_or_else(|_| Cell::Text(field.to_string()))
}
```
