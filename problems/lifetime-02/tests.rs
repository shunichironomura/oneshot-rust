use crate::judge::{case, check_eq};
use crate::user::Tokenizer;

fn collect(input: &str) -> Vec<&str> {
    let mut t = Tokenizer::new(input);
    let mut tokens = Vec::new();
    while let Some(tok) = t.next_token() {
        tokens.push(tok);
    }
    tokens
}

pub fn run() {
    case("example-1", || check_eq(collect("let x = 1;"), vec!["let", "x", "=", "1;"]));
    case("case-1", || check_eq(collect("   "), Vec::<&str>::new()));
    case("case-2", || {
        let owned = String::from(" a\tbb  ccc ");
        let tokens = collect(&owned);
        check_eq(tokens, vec!["a", "bb", "ccc"])
    });
}
