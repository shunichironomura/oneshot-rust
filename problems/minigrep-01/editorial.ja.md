`args` はどんなイテレータでもよいので、`len()` もなく、添字でアクセスすることもできません。使えるのは、
1 つずつ取り出す `next()` だけです。`next` は `&mut self` を取るので、引数は `mut args` と宣言する必要があります。
この `mut` はシグネチャではなくローカルな束縛の一部なので、呼び出し側からは見えません。`next()` が返す
`Option<String>` の中の `String` はイテレータからムーブされるので、`query` と `file_path` は `clone` せずに
そのまま受け取れます。`ok_or(...)?` で `None` をエラーにして、早期リターンできます。

エラーメッセージは文字列リテラルで、型は `&'static str` なので、そのまま返せます。4 つ目の要素については、
`String` を `Some("--ignore-case")` のようなリテラルのパターンとマッチさせることはできません（型エラーになります）。
`as_deref()` で `Option<String>` を `Option<&str>` に変えてからマッチさせます。`Some(arg) if arg == "--ignore-case"`
のようなガードでも書けます。Book の版は余分な引数を無視するので、5 つ目の要素がないことも忘れずに確かめてください。

`str::lines` は `\r\n` も最後の改行も正しく扱います。`split('\n')` では `\r` が残り、最後の改行の後に余分な空行ができます。

大文字と小文字を区別しない検索では、`query` をループの前に 1 回だけ小文字にします。`to_ascii_lowercase` ではなく
`to_lowercase` を使ってください。`to_ascii_lowercase` は `É` をそのままにします。`to_lowercase` は新しい `String` を返し、
`contains(&query)` にはその参照を渡します。`&String` はパターンとして使えますが、`String` は使えません。
`push` するのは元の `line` で、`line.to_lowercase()` ではありません。小文字にした行は一時的な `String` で、`&'a str` では
ありません。その参照もループの外までは生きられません。シグネチャの `'a` は結果を `contents` だけに結び付けているので、
`query` を破棄した後でも結果は有効です。

### 別解

Book のように `match` 式と `for` ループで書く方法です。`search` も `line.contains(query)` を使って同じように書けます。

```rust
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = match args.next() {
            None => false,
            Some(arg) if arg == "--ignore-case" => true,
            Some(_) => return Err("Unexpected argument"),
        };

        match args.next() {
            None => Ok(Config { query, file_path, ignore_case }),
            Some(_) => Err("Unexpected argument"),
        }
    }
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
```

引数を `Vec` に集めてから、スライスの形でマッチさせることもできます。すべての場合がひと目でわかりますが、
スライスの中の文字列は借用なので `clone` が必要です。また、引数が多すぎる場合でもすべて読んでしまいます。

```rust
pub fn build(args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    let args: Vec<String> = args.collect();
    let (query, file_path, ignore_case) = match args.as_slice() {
        [] | [_] => return Err("Didn't get a query string"),
        [_, _] => return Err("Didn't get a file path"),
        [_, query, file_path] => (query, file_path, false),
        [_, query, file_path, flag] if flag == "--ignore-case" => (query, file_path, true),
        _ => return Err("Unexpected argument"),
    };
    Ok(Config { query: query.clone(), file_path: file_path.clone(), ignore_case })
}
```
