pub fn build<T>(order: &[usize], mut f: impl FnMut(usize) -> T) -> Vec<T> {
    let n = order.len();
    let mut v = Vec::with_capacity(n);
    // Every slot gets its value in the loop below, so the length can be set right away.
    unsafe { v.set_len(n) };
    for &i in order {
        v[i] = f(i);
    }
    v
}
