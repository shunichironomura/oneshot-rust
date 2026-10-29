use crate::judge::{case, check_eq};
use crate::user::{Token, classify};

fn quoted(text: &str) -> Token {
    Token::Quoted(text.to_string())
}

fn pair(before: &str, after: &str) -> Token {
    Token::Pair(before.to_string(), after.to_string())
}

fn word(tok: &str) -> Token {
    Token::Word(tok.to_string())
}

pub fn run() {
    case("example-1", || {
        check_eq(classify("'a:b'"), quoted("a:b"))?;
        check_eq(classify("'it's'"), word("'it's'"))
    });
    case("example-2", || {
        check_eq(classify("a:b:c"), pair("a", "b:c"))?;
        check_eq(classify("hello"), word("hello"))
    });
    case("case-1", || {
        // A lone quote is not a pair of quotes, and a run of quotes has one inside.
        check_eq(classify("'"), word("'"))?;
        check_eq(classify("''"), quoted(""))?;
        check_eq(classify("'''"), word("'''"))?;
        check_eq(classify("''''"), word("''''"))?;
        check_eq(classify("'x"), word("'x"))?;
        check_eq(classify("x'"), word("x'"))
    });
    case("case-2", || {
        // The text between the quotes is kept as it is, whatever it contains.
        check_eq(classify("'hello'"), quoted("hello"))?;
        check_eq(classify("'7'"), quoted("7"))?;
        check_eq(classify("' a  b '"), quoted(" a  b "))?;
        check_eq(classify("'日本語'"), quoted("日本語"))?;
        check_eq(classify("':'"), quoted(":"))?;
        check_eq(classify("\"hi\""), word("\"hi\""))
    });
    case("case-3", || {
        // Split at the first ':'; both parts must be non-empty.
        check_eq(classify("a::b"), pair("a", ":b"))?;
        check_eq(classify("12:30"), pair("12", "30"))?;
        check_eq(classify("'a:b"), pair("'a", "b"))?;
        check_eq(classify("'it's:x'"), pair("'it's", "x'"))?;
        check_eq(classify("名前:値"), pair("名前", "値"))?;
        check_eq(classify(":b"), word(":b"))?;
        check_eq(classify("a:"), word("a:"))?;
        check_eq(classify("é:"), word("é:"))?;
        check_eq(classify("':"), word("':"))?;
        check_eq(classify(":"), word(":"))?;
        check_eq(classify("::b"), word("::b"))
    });
    case("case-4", || {
        // Everything else is a word, including the empty string and tokens that start with a
        // multi-byte character.
        check_eq(classify(""), word(""))?;
        check_eq(classify(" 7"), word(" 7"))?;
        check_eq(classify("1_000"), word("1_000"))?;
        check_eq(classify("é'"), word("é'"))?;
        check_eq(classify("é12345"), word("é12345"))
    });
}
