`1.10.2` や `2.0.0-rc.1` のようなソフトウェアのバージョンには、小さな文法があります。下の型 `Version` に
`FromStr` を実装し、`s.parse::<Version>()` が正しい文字列だけを受け付けるようにしてください
（`Version` と `ParseVersionError` はエディタに入っています）。

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseVersionError;

impl FromStr for Version { /* type Err = ParseVersionError; */ }
```

正しい文字列は `MAJOR[.MINOR[.PATCH]][-PRE]` の形をしています。角かっこの部分は省略できます。

- `MAJOR`、`MINOR`、`PATCH` はそれぞれ 1 文字以上の ASCII 数字（`0` ～ `9`）で、値は `u64` に
  収まらなければなりません。先頭の 0 は許されます（`007` は 7 です）。`MINOR` や `PATCH` を省略すると
  0 になり、`-PRE` がなければフィールド `pre` は `None` になります。
- `PRE` はプレリリースのラベルで、1 文字以上です。各文字は ASCII の英字、ASCII 数字、`.` の
  いずれかです。書かれたとおりに `pre` に入れます。
- それ以外のものは許されません。文字列の前後の空白は取り除かず、ほかの文字があったり、空の部分が
  あったりすれば不正です。不正な文字列には `Err(ParseVersionError)` を返します。
- `Version` と `ParseVersionError` の定義は変えないでください。

### 例

```rust
"1.2".parse::<Version>()      // Ok(Version { major: 1, minor: 2, patch: 0, pre: None })
"3-rc.1".parse::<Version>()   // Ok(Version { major: 3, minor: 0, patch: 0, pre: Some("rc.1") })
"1.2.3.4".parse::<Version>()  // Err(ParseVersionError)
```

`""`、`"1..2"`、`"1.2-"`、`"v1"` もどれも不正です。
