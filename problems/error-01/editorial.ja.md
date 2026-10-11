`?` は早期リターンするだけではなく、その前に `From::from` でエラーを変換します。
`impl From<ParseIntError> for ConfigError` を書いておけば、`value.trim().parse()?` が `ParseIntError` を
`ConfigError::BadNumber` に自動で変換します。ほかの場合は `ok_or(...)?` で `=` がないことをエラーにできます。
`split_once('=')` は最初の `=` だけで分けます。

`to_string()` や `{}` が使うのは `Display` です。`self` で `match` して、バリアントごとに `write!` でメッセージを書きます。
中の `ParseIntError` も `Display` を実装しているので、`{e}` でそのメッセージを出力できます。

`HashMap::insert` は以前の値を返すので、キーがすでにあったかどうかがわかります。

### 別解

`From` を実装せずに、呼び出し側で `map_err` を使って変換する方法もあります。タプルバリアントの名前は関数としても使えるので、
`ConfigError::BadNumber` をそのまま渡せます。

```rust
let value: u32 = value.trim().parse().map_err(ConfigError::BadNumber)?;
```

同じ元のエラーを場所によって別のバリアントにしたいときに便利です。いつも同じバリアントにするなら、`From` を使うと呼び出し側が短くなります。
