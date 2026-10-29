use crate::judge::{case, check_eq};
use crate::user::Money;

fn m(cents: i64) -> Money {
    Money { cents }
}

pub fn run() {
    case("example-1", || {
        check_eq(m(123456).to_string(), "$1,234.56")?;
        check_eq(format!("{}", m(-5)), "-$0.05")?;
        check_eq(format!("{}", m(-100000000)), "-$1,000,000.00")
    });
    case("example-2", || {
        check_eq(format!("{:>10}", m(5)), "     $0.05")?;
        check_eq(format!("{:>8}", m(-100)), "  -$1.00")?;
        check_eq(format!("{:*<9}", m(5)), "$0.05****")?;
        check_eq(format!("{:^11}", m(5)), "   $0.05   ")
    });
    case("case-1", || {
        // Small amounts and the boundaries of the first group.
        check_eq(m(0).to_string(), "$0.00")?;
        check_eq(m(99_999).to_string(), "$999.99")?;
        check_eq(m(100_000).to_string(), "$1,000.00")
    });
    case("case-2", || {
        // Several separators, grouped from the right.
        check_eq(m(99_999_999).to_string(), "$999,999.99")?;
        check_eq(m(1_234_567_890).to_string(), "$12,345,678.90")?;
        check_eq(m(12_345_678_901).to_string(), "$123,456,789.01")
    });
    case("case-3", || {
        // Negative amounts, including ones under a dollar.
        check_eq(m(-99).to_string(), "-$0.99")?;
        check_eq(m(-100).to_string(), "-$1.00")?;
        check_eq(m(-123_456_789).to_string(), "-$1,234,567.89")
    });
    case("case-4", || {
        check_eq(m(i64::MAX).to_string(), "$92,233,720,368,547,758.07")?;
        check_eq(m(i64::MIN).to_string(), "-$92,233,720,368,547,758.08")
    });
    case("case-5", || {
        // Fill and alignment; the sign is part of the padded text.
        check_eq(format!("{:-^10}", m(5)), "--$0.05---")?;
        check_eq(format!("{:=^9}", m(-100)), "=-$1.00==")?;
        check_eq(format!("{:_>12}", m(-123_456)), "__-$1,234.56")
    });
    case("case-6", || {
        // No alignment means left; a width not larger than the text adds nothing.
        check_eq(format!("[{}|{:>7}|{:6}]", m(1), m(-2), m(300)), "[$0.01| -$0.02|$3.00 ]")?;
        check_eq(format!("{:3}", m(123_456)), "$1,234.56")
    });
}
