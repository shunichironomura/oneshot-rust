use crate::judge::{case, check_eq};
use crate::user::{LexError, Token, tokenize};

type Lexed = Result<Vec<Token>, LexError>;

fn unexpected(ch: char, pos: usize) -> Lexed {
    Err(LexError::Unexpected { ch, pos })
}

pub fn run() {
    use Token::{Arrow, Num, Op};

    case("sample-1", || {
        check_eq(tokenize("1 -> 42+3"), Ok(vec![Num(1), Arrow, Num(42), Op('+'), Num(3)]))
    });
    case("sample-2", || {
        check_eq(tokenize("5-2"), Ok(vec![Num(5), Op('-'), Num(2)]))?;
        check_eq(tokenize("007*(1)"), Ok(vec![Num(7), Op('*'), Op('('), Num(1), Op(')')]))
    });
    case("sample-3", || check_eq(tokenize("1 >= 1"), unexpected('>', 2)));
    case("hidden-1", || {
        // Empty input and whitespace, including non-ASCII whitespace.
        check_eq(tokenize(""), Ok(vec![]))?;
        check_eq(
            tokenize("  1\t\u{3000}(2\u{a0})\n"),
            Ok(vec![Num(1), Op('('), Num(2), Op(')')]),
        )
    });
    case("hidden-2", || {
        // '-' is an arrow only when '>' follows immediately.
        check_eq(tokenize("1-"), Ok(vec![Num(1), Op('-')]))?;
        check_eq(tokenize("-->1->2"), Ok(vec![Op('-'), Arrow, Num(1), Arrow, Num(2)]))?;
        check_eq(tokenize("- >"), unexpected('>', 2))?;
        check_eq(tokenize("->>"), unexpected('>', 2))
    });
    case("hidden-3", || {
        // u64::MAX, leading zeros, and numbers directly followed by other tokens.
        check_eq(tokenize("18446744073709551615"), Ok(vec![Num(u64::MAX)]))?;
        check_eq(
            tokenize("000000000000000000042 9= 12-3"),
            Ok(vec![Num(42), Num(9), Op('='), Num(12), Op('-'), Num(3)]),
        )
    });
    case("hidden-4", || {
        // Every operator.
        check_eq(
            tokenize("(1+2)*3/4=5"),
            Ok(vec![
                Op('('),
                Num(1),
                Op('+'),
                Num(2),
                Op(')'),
                Op('*'),
                Num(3),
                Op('/'),
                Num(4),
                Op('='),
                Num(5),
            ]),
        )
    });
    case("hidden-5", || {
        // Letters and non-ASCII digits are errors; positions count characters, not bytes.
        check_eq(tokenize("12a"), unexpected('a', 2))?;
        check_eq(tokenize("1\u{663}"), unexpected('\u{663}', 1))?;
        check_eq(tokenize("\u{3000}1\u{3000}?"), unexpected('?', 3))?;
        check_eq(tokenize("\u{3000}é"), unexpected('é', 1))
    });
    case("hidden-6", || {
        // Only the first error is reported.
        check_eq(tokenize("(1 + 2) ? x"), unexpected('?', 8))?;
        check_eq(tokenize("1.5"), unexpected('.', 1))
    });
}
