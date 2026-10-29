use crate::judge::{case, check_eq};
use crate::user::merge_accounts;

fn merged(balances: &[u64], from: usize, to: usize) -> Vec<u64> {
    let mut v = balances.to_vec();
    merge_accounts(&mut v, from, to);
    v
}

pub fn run() {
    case("example-1", || check_eq(merged(&[5, 0, 7], 0, 2), vec![0, 0, 12]));
    case("case-1", || {
        // Merging an account into itself leaves everything unchanged.
        check_eq(merged(&[4, 9], 1, 1), vec![4, 9])?;
        check_eq(merged(&[4, 9], 0, 0), vec![4, 9])?;
        check_eq(merged(&[6], 0, 0), vec![6])
    });
    case("case-2", || {
        // `from` may come after `to`, and the accounts may be next to each other.
        check_eq(merged(&[1, 2, 3, 4], 3, 0), vec![5, 2, 3, 0])?;
        check_eq(merged(&[1, 2, 3, 4], 1, 2), vec![1, 0, 5, 4])?;
        check_eq(merged(&[1, 2, 3, 4], 2, 1), vec![1, 5, 0, 4])?;
        check_eq(merged(&[0, 8], 0, 1), vec![0, 8])
    });
}
