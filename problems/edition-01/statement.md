The code below does not compile. Fix it.

The code was written for Rust 2021 and compiled there, but the judge compiles it as Rust 2024. It
contains a small random number generator, a word classifier and a scoring function.

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
pub fn score(points: Box<[u32]>) -> u32;
```

- `Rng` is a 64-bit linear congruential generator. `Rng::new(seed)` starts with the state `seed`.
  Each call to `gen` replaces the state with `state * 6364136223846793005 + 1442695040888963407`
  (wrapping around on overflow) and returns the new state shifted right by 33 bits,
  `(state >> 33) as u32`.
- `gen_below(bound)` draws one number with `gen` and returns the remainder of dividing it by
  `bound`. `bound` is at least 1.
- `classify(word)` returns a `Token` whose `text` is `word`. Its `type` is `Kind::Keyword` if `word`
  is exactly one of `fn`, `let`, `match`, `gen`, `async` and `try` (case-sensitive), and
  `Kind::Ident` otherwise.
- `score(points)` returns the sum of the entries plus 1 for each entry, so an empty slice scores
  `0`. The result always fits in a `u32`.

Do not change the public names or signatures: the harness, which is also compiled as Rust 2024,
uses them.

### Examples

- The first three calls to `gen` on `Rng::new(0)` return `167951807`, `218396424` and `1299921937`.
- `classify("gen")` is `Token { r#type: Kind::Keyword, text: "gen".to_string() }`, and
  `classify("gene").r#type` is `Kind::Ident`.
- `score(vec![3, 4].into_boxed_slice())` is `3 + 4 + 2 = 9`.
