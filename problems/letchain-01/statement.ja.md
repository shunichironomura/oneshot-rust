あるコマンドラインツールは、読み込んだトークンを種類ごとに仕分けます。1 つのトークンの種類を判定する
`classify` と、2 つのバイトをオーバーフローさせずに足す小さな補助関数 `sum_first_two` を実装してください。
列挙型 `Token` は用意されています（エディタに入っています）。

```rust
#[derive(Debug, PartialEq)]
pub enum Token {
    Tiny(u64),
    Small(u64),
    Large(u64),
    Quoted(String),
    Pair(String, String),
    Word(String),
}

pub fn classify(tok: &str) -> Token
pub fn sum_first_two(nums: &[u8]) -> Option<u8>
```

`classify(tok)` は、次の規則のうち最初に当てはまるものを使います。`tok` はどんな文字列でもよく、
そのまま使います（前後の空白は取り除きません）。

1. `tok.parse::<u64>()` が成功して値 `n` が得られたら、トークンは数です。`n < 10` なら `Tiny(n)`、
   `10 <= n < 1000` なら `Small(n)`、`n >= 1000` なら `Large(n)` です。先頭の `+` や 0 は受け付けます
   （`"+7"` も `"007"` も 7 です）が、`"-7"`、`" 7"`、`"7.0"` や `u64::MAX` より大きい数は受け付けません。
2. `tok` が、開きのシングルクォート `'`、`'` を含まないテキスト `text`、閉じの `'` をこの順に
   つなげたものなら、トークンは `Quoted(text)` です。`text` はそのまま使います。空でもよく
   （`''` は `Quoted("")`）、`:` を含めてほかのどんな文字を含んでいても構いません。
3. `tok` に `:` が含まれていたら、**最初の** `:` で 2 つに分けます。`:` より前の部分 `before` と
   後ろの部分 `after` がどちらも空でなければ、トークンは `Pair(before, after)` です。`:` 自体は
   どちらの部分にも含めません。
4. どれにも当てはまらなければ、トークンは `Word(tok)` です。空文字列もこれに当たります。

`sum_first_two(nums)` は、`nums` の最初の 2 つの要素を `a`、`b` として `Some(a + b)` を返します。
`nums` の要素が 2 つより少ないときや、`a + b` が `u8::MAX`（255）より大きいときは `None` を返します。
3 つ目以降の要素は無視します。

### 例

| 呼び出し | 結果 |
| --- | --- |
| `classify("7")` | `Tiny(7)` |
| `classify("+250")` | `Small(250)` |
| `classify("'a:b'")` | `Quoted("a:b")` |
| `classify("'it's'")` | `Word("'it's'")`（3 つ目の `'` があります） |
| `classify("a:b:c")` | `Pair("a", "b:c")` |
| `sum_first_two(&[200, 55, 9])` | `Some(255)` |
| `sum_first_two(&[200, 56])` | `None`（`256` は `u8` に収まりません） |

`Pair("a", "b:c")` は `Token::Pair("a".to_string(), "b:c".to_string())` を短く書いたものです。
