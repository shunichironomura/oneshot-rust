`key = value` という行が並んだ設定ファイルを読み込んでマップにしてください。読み込めないときは、何が悪いのかを報告します。

```rust
#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingEquals { line: usize },
    BadNumber(ParseIntError),
    Duplicate(String),
}

impl fmt::Display for ConfigError { /* 実装する */ }

pub fn parse_config(text: &str) -> Result<HashMap<String, u32>, ConfigError>
```

- どの行も**最初の** `=` で分けます。キーと値の前後の空白は取り除きます。値は `u32` として解釈できなければなりません。
- 行番号は 1 から始まります。
- 最初に見つかった問題について、対応するエラーを返します。

| 問題 | エラー | `to_string()` |
| --- | --- | --- |
| 3 行目に `=` がない | `MissingEquals { line: 3 }` | `line 3: missing '='` |
| 値が `u32` でない | `BadNumber(e)`（`e` は `parse` が返したエラー） | `bad number: <e>` |
| キー `port` が 2 回出てくる | `Duplicate("port")` | `duplicate key: port` |

`<e>` は `ParseIntError` のメッセージです（例：`invalid digit found in string`）。

### 例

```rust
parse_config("port = 8080\nworkers=4\n")
// Ok({"port": 8080, "workers": 4})

parse_config("port = 8080\nverbose\n")
// Err(MissingEquals { line: 2 })。表示は "line 2: missing '='"
```
