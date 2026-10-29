use crate::judge::{case, check_eq};
use crate::user::Labels;

pub fn run() {
    case("example-1", || {
        let mut labels = Labels::new();
        let first = labels.get(3).to_string();
        let second = labels.get(3).to_string();
        check_eq((first.as_str(), second.as_str(), labels.built()), ("item-3", "item-3", 1))
    });
    case("case-1", || check_eq(Labels::new().built(), 0));
    case("case-2", || {
        let mut labels = Labels::new();
        let got: Vec<String> =
            [1, 2, 1, 0, 2, 1].iter().map(|&id| labels.get(id).to_string()).collect();
        let expected = ["item-1", "item-2", "item-1", "item-0", "item-2", "item-1"];
        check_eq((got, labels.built()), (expected.map(String::from).to_vec(), 3))
    });
    case("case-3", || {
        let mut labels = Labels::new();
        let len = labels.get(u32::MAX).len();
        let label = labels.get(u32::MAX).to_string();
        check_eq((label.as_str(), len, labels.built()), ("item-4294967295", 15, 1))
    });
}
