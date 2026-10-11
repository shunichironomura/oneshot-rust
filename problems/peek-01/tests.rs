use crate::judge::{case, check_eq};
use crate::user::{LexError, Token, tokenize};

type Lexed = Result<Vec<Token>, LexError>;

fn ident(name: &str) -> Token {
    Token::Ident(name.to_string())
}

fn unexpected(ch: char, pos: usize) -> Lexed {
    Err(LexError::Unexpected { ch, pos })
}

fn overflow(pos: usize) -> Lexed {
    Err(LexError::Overflow { pos })
}

pub fn run() {
    use Token::{Arrow, Num, Op};

    case("sample-1", || {
        check_eq(
            tokenize("x1 -> 42+y"),
            Ok(vec![ident("x1"), Arrow, Num(42), Op('+'), ident("y")]),
        )
    });
    case("sample-2", || {
        check_eq(tokenize("a-b"), Ok(vec![ident("a"), Op('-'), ident("b")]))?;
        check_eq(tokenize("3x"), Ok(vec![Num(3), ident("x")]))
    });
    case("sample-3", || check_eq(tokenize("a >= 1"), unexpected('>', 2)));
    case("hidden-1", || {
        // Empty input and whitespace, including non-ASCII whitespace.
        check_eq(tokenize(""), Ok(vec![]))?;
        check_eq(
            tokenize("  a\t\u{3000}(b\u{a0})\n"),
            Ok(vec![ident("a"), Op('('), ident("b"), Op(')')]),
        )
    });
    case("hidden-2", || {
        // '-' is an arrow only when '>' follows immediately.
        check_eq(tokenize("x-"), Ok(vec![ident("x"), Op('-')]))?;
        check_eq(tokenize("-->1->2"), Ok(vec![Op('-'), Arrow, Num(1), Arrow, Num(2)]))?;
        check_eq(tokenize("- >"), unexpected('>', 2))?;
        check_eq(tokenize("->>"), unexpected('>', 2))
    });
    case("hidden-3", || {
        // u64::MAX fits, one more overflows; the position counts characters.
        check_eq(tokenize("18446744073709551615"), Ok(vec![Num(u64::MAX)]))?;
        check_eq(tokenize("x\u{3000}= 18446744073709551616"), overflow(4))
    });
    case("hidden-4", || {
        // Leading zeros, and numbers directly followed by other tokens.
        check_eq(
            tokenize("000000000000000000042 9_ 12ab3"),
            Ok(vec![Num(42), Num(9), ident("_"), Num(12), ident("ab3")]),
        )
    });
    case("hidden-5", || {
        // Identifiers and every operator.
        check_eq(
            tokenize("_ _a_1 B2c __"),
            Ok(vec![ident("_"), ident("_a_1"), ident("B2c"), ident("__")]),
        )?;
        check_eq(
            tokenize("(a+b)*c/d=e"),
            Ok(vec![
                Op('('),
                ident("a"),
                Op('+'),
                ident("b"),
                Op(')'),
                Op('*'),
                ident("c"),
                Op('/'),
                ident("d"),
                Op('='),
                ident("e"),
            ]),
        )
    });
    case("hidden-6", || {
        // Only ASCII letters and digits; positions count characters, not bytes.
        check_eq(tokenize("café"), unexpected('é', 3))?;
        check_eq(tokenize("x\u{663}"), unexpected('\u{663}', 1))?;
        check_eq(tokenize("\u{3000}x\u{3000}?"), unexpected('?', 3))
    });
    case("hidden-7", || {
        // Only the first error is reported. Numbers can be longer than any integer type.
        check_eq(tokenize("(1000000000000000000000000000000000000000) ?"), overflow(1))?;
        check_eq(tokenize("a.b"), unexpected('.', 1))
    });
}
