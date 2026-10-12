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
}

impl fmt::Display for ConfigError { /* 実装する */ }
impl Error for ConfigError { /* 実装する */ }

pub fn parse_port(s: &str) -> Result<u16, ConfigError>
pub fn load(pairs: &[(&str, &str)]) -> Result<(String, u16, u64), Box<dyn Error>>
pub fn report(e: &dyn Error) -> String
```

`ConfigError`（列挙型はエディタに入っています）：

- `Missing(key)` は `missing key: <key>`、`BadPort { value, .. }` は `bad port '<value>'` と
  表示します。`BadPort` のメッセージには、中の `ParseIntError` のメッセージを含めません。
- `source()` は、`BadPort` なら中の `ParseIntError` を返し、`Missing` なら `None` を返します。

`parse_port(s)` は、`s` を前後の空白を取り除かずに、`str::parse` で `u16` として解釈します。
失敗したら、エラーは `BadPort { value, source }` です。`value` は `s`、`source` は `parse` が返した
`ParseIntError` です。

`load(pairs)`：

- 次の順にキーを調べ、最初に見つかった問題を返します。
  1. `host` は必須で、ないときのエラーは `Missing("host")` です。
  2. `port` は必須で、ないときのエラーは `Missing("port")` です。値は `parse_port` で読み、
     `parse_port` のエラーはそのまま返します。
  3. `timeout` は省略でき、省略時は `30` です。値も前後の空白を取り除かずに、
     `str::parse` で `u64` として解釈します。失敗したときのエラーは `ParseIntError` そのものです。
     `ConfigError` には包みません。
- 結果は `(host, port, timeout)` です。エラーはボックスに入れて返します。テストは
  `downcast_ref::<ConfigError>()` と `downcast_ref::<ParseIntError>()` でエラーの型を確かめます。

`report(e)` は、このモジュールのエラーに限らず、どんなエラーにも使えます。`e` とその `source` の
連鎖にあるすべてのエラーのメッセージ（`to_string()`）を、順に `": "` でつないで返します。

### 例

```rust
load(&[("host", "h"), ("port", "8080")])  // Ok(("h".to_string(), 8080, 30))
parse_port("70000")  // Err(BadPort { value: "70000", source: .. })。u16 には大きすぎる

let e = load(&[("host", "h"), ("port", "x1")]).unwrap_err();
e.to_string()                     // "bad port 'x1'"
e.downcast_ref::<ConfigError>()   // Some(BadPort { value: "x1", source: .. })
e.source().unwrap().to_string()   // "invalid digit found in string"（ParseIntError）
report(&*e)                       // "bad port 'x1': invalid digit found in string"

let e = load(&[("host", "h"), ("port", "80"), ("timeout", "-1")]).unwrap_err();
e.downcast_ref::<ParseIntError>() // Some(..)。e.downcast_ref::<ConfigError>() は None
report(&*e)                       // "invalid digit found in string"
```
