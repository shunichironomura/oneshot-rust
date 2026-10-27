次のコードはコンパイルが通りません。直してください。

3 つの関数は、それぞれ本体にリテラルで書かれた決まった値を 1 つ返します。

```rust
pub fn number_pattern() -> &'static str
pub fn hashtag_quote() -> &'static str
pub fn png_magic() -> &'static [u8]
```

テストは、各関数の結果を下の表の値と 1 文字ずつ（`png_magic` は 1 バイトずつ）正確に比べます。
表の値は Rust のリテラルではなく、文字そのものです。表の `\` はどれもバックスラッシュ 1 文字です。

| 関数 | 値 | `len()` |
| --- | --- | --- |
| `number_pattern` | `\d+\.\d+`（`3.14` のような数を表す正規表現） | 8 |
| `hashtag_quote` | `He said "#rust" twice` | 21 |
| `png_magic` | PNG ファイルのシグネチャ。バイト `0x89`、`0x50`、`0x4E`、`0x47`、`0x0D`、`0x0A`、`0x1A`、`0x0A` | 8 |

シグネチャは変えないでください。

### 例

- `number_pattern()` は `\`、`d`、`+`、`\`、`.`、`\`、`d`、`+` の 8 文字を返します。
