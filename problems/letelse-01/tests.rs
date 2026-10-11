use crate::judge::{case, check, check_eq};
use crate::user::{parse_orders, total_quantity};

// Keep the texts short: Miri is slow at string processing.
const EXAMPLE: &str = "3 cups\n# note\n\n 2 pens # red\nx cups\n0 pens\n4 big cups\n5 cups";

fn none() -> Vec<(u32, &'static str)> {
    Vec::new()
}

pub fn run() {
    case("sample-1", || check_eq(parse_orders(EXAMPLE), vec![(3, "cups"), (2, "pens"), (5, "cups")]));
    case("sample-2", || check_eq(total_quantity(EXAMPLE, "cups"), 8));
    case("hidden-1", || {
        // Empty text, blank lines and comment-only lines.
        check_eq(parse_orders(""), none())?;
        check_eq(parse_orders(" \n\t# 3 a\n#"), none())?;
        check_eq(total_quantity("", "cups"), 0)
    });
    case("hidden-2", || {
        // COUNT is whatever parse::<u32> accepts, except 0. 2^32 + 1 must not wrap around to 1.
        let text = "+7 a\n-7 b\n007 c\n00 d\n4294967297 e\n4294967295 f";
        check_eq(parse_orders(text), vec![(7, "a"), (7, "c"), (4294967295, "f")])
    });
    case("hidden-3", || {
        // Any amount of spaces and tabs, and \r\n line endings.
        let text = "3\tcups\r\n  7   pens \t\r\n\t1  a";
        check_eq(parse_orders(text), vec![(3, "cups"), (7, "pens"), (1, "a")])
    });
    case("hidden-4", || {
        // Exactly two words, in the order COUNT ITEM.
        check_eq(parse_orders("2 a b\n5\na 5\n1 ok"), vec![(1, "ok")])
    });
    case("hidden-5", || {
        // The first '#', anywhere in the line, starts a comment. The items are slices of the text.
        let text = String::from("3 cups # x # y\n4 pens#red\n5 #a\n1 a#b c");
        let orders = parse_orders(&text);
        check_eq(&orders, &vec![(3, "cups"), (4, "pens"), (1, "a")])?;
        let offsets: Vec<usize> =
            orders.iter().map(|(_, item)| item.as_ptr().addr().wrapping_sub(text.as_ptr().addr())).collect();
        check(offsets == [2, 17, 33], "items must be slices of text")
    });
    case("hidden-6", || {
        // total_quantity matches the item exactly and ignores invalid lines.
        check_eq(total_quantity("2 cups\n3 cup\n4 Cups\n5 cups#x\n1 teacups\n6 cups a", "cups"), 7)
    });
}
