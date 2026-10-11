/// A small deterministic random number generator (a 64-bit LCG).
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng { state: seed }
    }

    pub fn r#gen(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.state >> 33) as u32
    }

    /// `bound` must be at least 1.
    pub fn gen_below(&mut self, bound: u32) -> u32 {
        self.r#gen() % bound
    }
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

pub fn classify(word: &str) -> Token {
    let r#type = match word {
        "fn" | "let" | "match" | "gen" | "async" | "try" => Kind::Keyword,
        _ => Kind::Ident,
    };
    Token { r#type, text: word.to_string() }
}

/// Each entry is worth its points plus a bonus of 1.
pub fn score(points: Box<[u32]>) -> u32 {
    let mut total = 0;
    for p in points.iter() {
        total += *p;
    }
    total + points.len() as u32
}
