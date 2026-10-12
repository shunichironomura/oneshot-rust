あるコマンドラインツールは、読み込んだトークンを種類ごとに仕分けます。1 つのトークンの種類を判定する
`classify` を実装してください。列挙型 `Token` は用意されています（エディタに入っています）。

```rust
#[derive(Debug, PartialEq)]
pub enum Token {
    Quoted(String),
    Pair(String, String),
    Word(String),
}

pub fn classify(tok: &str) -> Token
```

`classify(tok)` は、次の規則のうち最初に当てはまるものを使います。`tok` はどんな文字列でもよく、
そのまま使います（前後の空白は取り除きません）。

1. `tok` が、開きのシングルクォート `'`、`'` を含まないテキスト `text`、閉じの `'` をこの順に
   つなげたものなら、トークンは `Quoted(text)` です。`text` はそのまま使います。空でもよく
   （`''` は `Quoted("")`）、`:` を含めてほかのどんな文字を含んでいても構いません。
2. `tok` に `:` が含まれていたら、**最初の** `:` で 2 つに分けます。`:` より前の部分 `before` と
   後ろの部分 `after` がどちらも空でなければ、トークンは `Pair(before, after)` です。`:` 自体は
   どちらの部分にも含めません。
3. どれにも当てはまらなければ、トークンは `Word(tok)` です。空文字列もこれに当たります。

### 例

| 呼び出し | 結果 |
| --- | --- |
| `classify("'a:b'")` | `Quoted("a:b")` |
| `classify("'it's'")` | `Word("'it's'")`（3 つ目の `'` があります） |
| `classify("a:b:c")` | `Pair("a", "b:c")` |
| `classify("hello")` | `Word("hello")` |

`Pair("a", "b:c")` は `Token::Pair("a".to_string(), "b:c".to_string())` を短く書いたものです。
