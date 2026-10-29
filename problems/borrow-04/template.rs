pub fn merge_accounts(balances: &mut [u64], from: usize, to: usize) {
    let src = &mut balances[from];
    let dst = &mut balances[to];
    *dst += *src;
    *src = 0;
}
