use crate::judge::{case, check_eq};
use crate::user::{Token, classify, sum_first_two};

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
    case("sample-1", || {
        check_eq(classify("7"), Token::Tiny(7))?;
        check_eq(classify("+250"), Token::Small(250))
    });
    case("sample-2", || {
        check_eq(classify("'a:b'"), quoted("a:b"))?;
        check_eq(classify("'it's'"), word("'it's'"))?;
        check_eq(classify("a:b:c"), pair("a", "b:c"))
    });
    case("sample-3", || {
        check_eq(sum_first_two(&[200, 55, 9]), Some(255))?;
        check_eq(sum_first_two(&[200, 56]), None)
    });
    case("hidden-1", || {
        // The boundaries between the three kinds of numbers, and the largest u64.
        check_eq(classify("0"), Token::Tiny(0))?;
        check_eq(classify("9"), Token::Tiny(9))?;
        check_eq(classify("10"), Token::Small(10))?;
        check_eq(classify("999"), Token::Small(999))?;
        check_eq(classify("1000"), Token::Large(1000))?;
        check_eq(classify("007"), Token::Tiny(7))?;
        check_eq(classify("+0999"), Token::Small(999))?;
        check_eq(classify("18446744073709551615"), Token::Large(u64::MAX))
    });
    case("hidden-2", || {
        // Tokens that parse::<u64> rejects are not numbers.
        check_eq(classify("-7"), word("-7"))?;
        check_eq(classify(" 7"), word(" 7"))?;
        check_eq(classify("7.0"), word("7.0"))?;
        check_eq(classify("18446744073709551616"), word("18446744073709551616"))?;
        check_eq(classify("+"), word("+"))?;
        check_eq(classify(""), word(""))
    });
    case("hidden-3", || {
        // A lone quote is not a pair of quotes, and a run of quotes has one inside.
        check_eq(classify("'"), word("'"))?;
        check_eq(classify("''"), quoted(""))?;
        check_eq(classify("'''"), word("'''"))?;
        check_eq(classify("''''"), word("''''"))?;
        check_eq(classify("'x"), word("'x"))?;
        check_eq(classify("x'"), word("x'"))
    });
    case("hidden-4", || {
        // The text between the quotes is kept as it is, whatever it contains.
        check_eq(classify("'hello'"), quoted("hello"))?;
        check_eq(classify("'7'"), quoted("7"))?;
        check_eq(classify("' a  b '"), quoted(" a  b "))?;
        check_eq(classify("'日本語'"), quoted("日本語"))?;
        check_eq(classify("':'"), quoted(":"))?;
        check_eq(classify("\"hi\""), word("\"hi\""))
    });
    case("hidden-5", || {
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
    case("hidden-6", || {
        // Everything else is a word, including tokens that start with a multi-byte character.
        check_eq(classify("hello"), word("hello"))?;
        check_eq(classify("1_000"), word("1_000"))?;
        check_eq(classify("é'"), word("é'"))?;
        check_eq(classify("é12345"), word("é12345"))
    });
    case("hidden-7", || {
        // Fewer than two elements, sums around 255, and extra elements.
        check_eq(sum_first_two(&[]), None)?;
        check_eq(sum_first_two(&[7]), None)?;
        check_eq(sum_first_two(&[0, 0]), Some(0))?;
        check_eq(sum_first_two(&[255, 0]), Some(255))?;
        check_eq(sum_first_two(&[1, 2, 250]), Some(3))?;
        check_eq(sum_first_two(&[128, 128]), None)?;
        check_eq(sum_first_two(&[255, 255, 0]), None)
    });
}
