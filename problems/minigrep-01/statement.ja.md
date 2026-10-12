Rust Book の 12 章と 13 章で作る検索ツール `minigrep` の中心部分を作ってください。ここには本物のファイルも
環境変数もありません。コマンドライン引数は `String` のイテレータとして、ファイルの中身は文字列として渡されます。
構造体 `Config` は用意されています（エディタに入っています）。

```rust
#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config, &'static str>;
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>;
```

`Config::build` は引数を先頭から順に読みます。

1. 1 つ目の要素はプログラム名で、使いません。
2. 2 つ目の要素が `query` です。2 つ目の要素がなければ（イテレータが空の場合も含みます）、
   `Err("Didn't get a query string")` を返します。
3. 3 つ目の要素が `file_path` です。なければ `Err("Didn't get a file path")` を返します。
4. 4 つ目の要素は省略できます。ちょうど `--ignore-case` であれば `ignore_case` は `true` です。4 つ目の要素が
   なければ `false` です。それ以外の 4 つ目の要素や、5 つ目の要素があるときは `Err("Unexpected argument")` を
   返します。

意味を持つのは要素の位置だけです。空文字列も 1 つの要素として数えます。ほかの位置にある `--ignore-case` は
ふつうの値として扱います。

`search` は、`contents` の行のうち `query` を含むものを、元の順番で返します。

- 行の分け方は `str::lines` と同じです。行は `\n` または `\r\n` で終わり、行末の改行は行に含めません。
  `contents` が改行で終わっていても、その後ろに空の行があるとはみなしません。`contents` が空なら行はありません。
- 大文字と小文字は区別します。`query` を何回含んでいても、その行を返すのは 1 回だけです。同じ内容の行が
  複数あれば、それぞれ返します。空の `query` は、空行も含めてすべての行に含まれます。

返す行は `contents` だけを借用します。`query` を破棄した後でも使えます。

### 例

引数が `minigrep to poem.txt` のとき、`Config::build` は
`Ok(Config { query: "to", file_path: "poem.txt", ignore_case: false })` を返します（フィールドは `String` です）。

```rust
let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
search("duct", contents) // ["safe, fast, productive."]
search("rust", contents) // ["Trust me."]
```
