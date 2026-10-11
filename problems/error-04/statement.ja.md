サーバーが `(キー, 値)` のペアから設定を読み込みます。設定が間違っているときのエラーは、その原因を
保持します。たとえばポートが不正なときのエラーは、数値として解釈できなかった理由を表す
`ParseIntError` を包んでいます。呼び出し側は `Error::source` をたどって、エラーからその原因に進めます。

```rust
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    Missing(&'static str),
    BadPort { value: String, source: ParseIntError },
    OutOfRange(u32),
}

impl fmt::Display for ConfigError { /* 実装する */ }
impl Error for ConfigError { /* 実装する */ }

pub fn parse_port(s: &str) -> Result<u16, ConfigError>
pub fn load(pairs: &[(&str, &str)]) -> Result<(String, u16, u64), Box<dyn Error>>
pub fn root_cause<'a>(e: &'a (dyn Error + 'static)) -> &'a (dyn Error + 'static)
pub fn report(e: &dyn Error) -> String
```

`ConfigError`（列挙型はエディタに入っています）：

- `Missing(key)` は `missing key: <key>`、`BadPort { value, .. }` は `bad port '<value>'`、
  `OutOfRange(n)` は `port <n> out of range` と表示します。`BadPort` のメッセージには、
  中の `ParseIntError` のメッセージを含めません。
- `source()` は、`BadPort` なら中の `ParseIntError` を返し、ほかのバリアントなら `None` を返します。

`parse_port(s)`：

- `s` を前後の空白を取り除かずに、`str::parse` で `u32` として解釈します。失敗したら、エラーは
  `BadPort { value, source }` です。`value` は `s`、`source` は `parse` が返した `ParseIntError` です。
- 数が `0` のときと `65535` より大きいときは `OutOfRange(n)` です。それ以外の数はそのまま
  ポート番号です。

`load(pairs)`：

- キーは完全一致で比べます（大文字と小文字を区別し、空白も取り除きません）。ほかのキーのペアは無視します。
  同じキーが複数回出てくるときは、最初のペアだけを使います。
- 次の順にキーを調べ、最初に見つかった問題を返します。
  1. `host` は必須で、ないときのエラーは `Missing("host")` です。値は空文字列でもそのまま使います。
  2. `port` は必須で、ないときのエラーは `Missing("port")` です。値は `parse_port` で読み、
     `parse_port` のエラーはそのまま返します。
  3. `timeout` は省略でき、省略時は `30` です。値も前後の空白を取り除かずに、
     `str::parse` で `u64` として解釈します。失敗したときのエラーは `ParseIntError` そのものです。
     `ConfigError` には包みません。
- 結果は `(host, port, timeout)` です。エラーはボックスに入れて返します。テストは
  `downcast_ref::<ConfigError>()` と `downcast_ref::<ParseIntError>()` でエラーの型を確かめます。

`root_cause` と `report` は、このモジュールのエラーに限らず、どんなエラーにも使えます。

- `root_cause(e)` は `e` から `source()` をたどれる限りたどり、連鎖の最後のエラーを返します。
  `e` に `source` がなければ `e` 自身です。
- `report(e)` は、`e` とその `source` の連鎖にあるすべてのエラーのメッセージ（`to_string()`）を
  順に `": "` でつないで返します。

### 例

```rust
load(&[("host", "h"), ("port", "8080")])  // Ok(("h".to_string(), 8080, 30))
parse_port("70000")                      // Err(ConfigError::OutOfRange(70000))

let e = load(&[("host", "h"), ("port", "x1")]).unwrap_err();
e.to_string()                     // "bad port 'x1'"
e.downcast_ref::<ConfigError>()   // Some(BadPort { value: "x1", source: .. })
root_cause(&*e).to_string()       // "invalid digit found in string"（ParseIntError）
report(&*e)                       // "bad port 'x1': invalid digit found in string"

let e = load(&[("host", "h"), ("port", "80"), ("timeout", "-1")]).unwrap_err();
e.downcast_ref::<ParseIntError>() // Some(..)。e.downcast_ref::<ConfigError>() は None
report(&*e)                       // "invalid digit found in string"
```
