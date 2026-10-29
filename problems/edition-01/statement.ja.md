次のコードはコンパイルが通りません。直してください。

このコードは Rust 2021 向けに書かれ、そこではコンパイルが通っていました。しかし、ジャッジは
Rust 2024 としてコンパイルします。中身は、小さな乱数生成器と単語の分類の 2 つです。

```rust
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Rng;
    pub fn gen(&mut self) -> u32;
    pub fn gen_below(&mut self, bound: u32) -> u32;
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    Keyword,
    Ident,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub r#type: Kind,
    pub text: String,
}

pub fn classify(word: &str) -> Token;
```

- `Rng` は 64 ビットの線形合同法による乱数生成器です。`Rng::new(seed)` は状態 `seed` から始まります。
  `gen` は呼ばれるたびに、状態を `state * 6364136223846793005 + 1442695040888963407`
  （オーバーフローしたら折り返す）で置き換え、新しい状態を 33 ビット右にシフトした値
  `(state >> 33) as u32` を返します。
- `gen_below(bound)` は `gen` で数を 1 つ引き、それを `bound` で割った余りを返します。
  `bound` は 1 以上です。
- `classify(word)` は、`text` が `word` である `Token` を返します。`word` が `fn`、`let`、`match`、
  `gen`、`async`、`try` のどれかと完全に一致すれば（大文字と小文字は区別します）`type` は
  `Kind::Keyword`、そうでなければ `Kind::Ident` です。

公開されている名前とシグネチャは変えないでください。ハーネスも Rust 2024 としてコンパイルされ、
それらを使います。

### 例

- `Rng::new(0)` で `gen` を 3 回呼ぶと、`167951807`、`218396424`、`1299921937` を返します。
- `classify("gen")` は `Token { r#type: Kind::Keyword, text: "gen".to_string() }` で、
  `classify("gene").r#type` は `Kind::Ident` です。
