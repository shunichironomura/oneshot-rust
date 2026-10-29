pub fn merge_accounts(balances: &mut [u64], from: usize, to: usize) {
    // Both indices are in range, so this is `Err` only when `from == to`: nothing to do then.
    if let Ok([src, dst]) = balances.get_disjoint_mut([from, to]) {
        *dst += *src;
        *src = 0;
    }
}
