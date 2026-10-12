プログラムの設定を表す構造体 `Config` が用意されています（エディタに入っています）。`Config` に
`Default` を実装し、設定を作る 3 つの関数を実装してください。

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub name: String,
    pub retries: u32,
    pub verbose: bool,
    pub tags: Vec<String>,
}

impl Default for Config { /* 実装する */ }

pub fn renamed(base: &Config, name: &str) -> Config
pub fn quiet(base: Config) -> Config
pub fn with_tags(tags: &[&str]) -> Config
```

- `Config::default()` は、名前が `"app"`、`retries` が `3`、`verbose` が `false` で、タグは
  ありません。
- `renamed` は、`base` の名前を `name` に変えたコピーを返します。ほかのフィールドはすべて
  `base` と同じです。`base` 自体は借用されるだけで、元のままです。
- `quiet` は `base` を値で受け取り、`verbose` を `false`、`retries` を `0` にして返します。
  名前とタグはそのままです。
- `with_tags` は、タグが `tags` である既定の設定を返します。タグは与えられた順に並べ、
  重複もそのまま残します。

### 例

- `with_tags(&["a", "b"])` が返すのは、名前が `"app"`、`retries` が `3`、`verbose` が `false`、
  タグが `["a", "b"]` の設定です。
- `renamed(&with_tags(&["x"]), "db")` が返すのは、名前が `"db"`、`retries` が `3`、
  `verbose` が `false`、タグが `["x"]` の設定です。
