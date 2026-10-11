トレイト `Outline` は、値を `Display` で表示したときのテキストを `*` の枠で囲みます。トレイトと
2 つの型 `Point`・`Tags` が用意されています（エディタに入っています）。足りない本体を書いてください。

```rust
pub trait Outline {
    fn outline(&self) -> String { /* 実装する */ }
}

pub struct Point { pub x: i32, pub y: i32 }
pub struct Tags(pub Vec<String>);

impl fmt::Display for Point { /* 実装する */ }
impl fmt::Display for Tags { /* 実装する */ }

impl Outline for Point {}
impl Outline for Tags {}
```

- `Point` は `(`、`x`、コンマ、空白 1 つ、`y`、`)` の順に表示します。座標は 10 進数で書きます
  （`(1, 3)`、`(-4, 0)`）。
- `Tags` は `[`、すべての文字列を順に（空文字列も含めて）コンマと空白 1 つで区切ったもの、`]` の順に
  表示します（`[a, b, c]`）。文字列が 1 つもなければ `[]` と表示します。
- 値を `Display` で表示したテキストを `s`（改行は含みません）、`s` の文字数（`char` の個数）を
  `w` とします。`outline` は次の 5 行を `\n` でつないだ文字列を返します。最後に `\n` は付けません。
  1. `*` を `w + 4` 個並べたもの。
  2. `*`、`w + 2` 個の空白、`*`。
  3. `*`、空白 1 つ、`s`、空白 1 つ、`*`。
  4. 2 行目と同じ。
  5. 1 行目と同じ。
- `outline` はデフォルトメソッドです。`Point` と `Tags` の `impl Outline` ブロックは空のままにします。
  テストハーネスも `Display` を実装した独自の型を持っていて、その型にも空の
  `impl Outline for ... {}` を書きます。
- `Point` と `Tags` のフィールド、`outline` の引数（`&self`）と戻り値の型（`String`）は
  変えないでください。それ以外は必要に応じて変更してかまいません。

### 例

- `Point { x: 1, y: 3 }.to_string()` は `"(1, 3)"` です。
- `Tags(vec!["red".to_string(), "green".to_string(), "blue".to_string()]).to_string()` は
  `"[red, green, blue]"` です。`"solo"` だけを持つ `Tags` は `[solo]` と表示されます。
- `Point { x: 1, y: 3 }.outline()` は次の 5 行を返します。

```text
**********
*        *
* (1, 3) *
*        *
**********
```
