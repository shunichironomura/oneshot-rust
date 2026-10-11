次の 3 つの関数の本体はすでに書かれていますが、コードはコンパイルが通りません。
**シグネチャだけ**を書き換えて、ハーネス（`tests.rs` タブ）がコンパイルでき、テストに合格するようにしてください。
本体は変更しないでください。

```rust
pub fn strip_tag(line: &str, tag: &str) -> &str
pub fn pick(a: &str, b: &str, first: bool) -> &str
pub fn first_word(s: &str) -> &str
```

- `strip_tag` は `line` の先頭から `tag` を取り除き、その直後に続く空白を読み飛ばして、残りを返します。
  末尾の空白はそのまま残し、取り除く `tag` は 1 つだけです。`line` が `tag` で始まらない場合
  （大文字・小文字は区別し、`tag` の前の空白は読み飛ばしません）は、`line` そのものを変更せずに返します。
  返り値は常に `line` のスライスです。
- `pick` は `first` が `true` なら `a` そのものを、そうでなければ `b` そのものを返します。
- `first_word` は、空白（スペース、タブ、改行など）で区切られた `s` の最初の単語を返します。
  返り値は `s` のスライスです。`s` に単語がなければ `""` を返します。

### 例

- `strip_tag("[INFO]  server started", "[INFO]")` は `"server started"` です。
- `pick("left", "right", true)` は `"left"`、`pick("left", "right", false)` は `"right"` です。
- `first_word("  hello world")` は `"hello"`、`first_word("Rust is fun")` は `"Rust"` です。
