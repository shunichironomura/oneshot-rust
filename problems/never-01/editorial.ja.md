`field` は解釈する型 `T` についてジェネリックで、`T` は呼び出し側が決めます。`sum_ids` の
`let id: u32 = field(&cols, 0)?;` では型注釈が `T` を決めています。一方 `validate_row` には `T` を決めるものが
ありません。`field(&cols, 1)?` の値は捨てられ、返り値の型 `Result<(), BadField>` が決めるのはエラーの型だけです。

エラーメッセージが「type annotations needed」ではなく `!` についてのものになるのは、`?` の仕組みによります。
`?` はエラーのときに `return` する `match` に展開され、`return` の型は never 型 `!` です。このように発散するアームを
持つ `match` の型がほかの何にも制約されないとき、コンパイラはあきらめずに既定の型（フォールバック）を使います。
以前のフォールバックは `()` でしたが、Rust 1.100 からはどのエディションでも `!` です（Rust 2024 ではもともと `!`
でした）。`!` は `FromStr` を実装していないので、``the trait bound `!: FromStr` is not satisfied`` になります。
`?` を付けない `field(&cols, 1);` なら、いつもの「type annotations needed」になります。

直し方は、ターボフィッシュで型を指定することです（`field::<u8>(&cols, 1)?`）。どの型を指定するかがそのまま
検証の中身になるので、問題文のとおりの型にする必要があります。年齢を `u32` にすると `300` を受け入れ、ID を
`f64` にすると `1.5` を、`i32` にすると `-1` を受け入れます。`String` なら何でも通ってしまいます。`()` は
`FromStr` を実装していないので、`field::<()>` もコンパイルが通りません（つまり、フォールバックが `()` だった
ころも、このコードはコンパイルできませんでした）。

`let _ = field(&cols, 1)?;` では解決しません。`_` はパターンであって型ではないので、`T` は決まらないままで、
やはり `!` にフォールバックします。

### 別解

型注釈を付けたワイルドカードパターンなら `T` が決まります。

```rust
pub fn validate_row(line: &str) -> Result<(), BadField> {
    let cols: Vec<&str> = line.split(',').collect();
    let _: u32 = field(&cols, 0)?;
    let _: u8 = field(&cols, 1)?;
    let _: f64 = field(&cols, 2)?;
    Ok(())
}
```

タプル型で行全体に一度に注釈を付けることもできます。タプル式の要素は左から順に評価されるので、
列を調べる順番も変わりません。

```rust
pub fn validate_row(line: &str) -> Result<(), BadField> {
    let cols: Vec<&str> = line.split(',').collect();
    let _row: (u32, u8, f64) = (field(&cols, 0)?, field(&cols, 1)?, field(&cols, 2)?);
    Ok(())
}
```
