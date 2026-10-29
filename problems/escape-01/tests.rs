use crate::judge::{case, check_eq};
use crate::user::number_pattern;

pub fn run() {
    case("example-1", || {
        check_eq(number_pattern(), r"\d+\.\d+")?;
        check_eq(number_pattern().len(), 8)
    });
}
