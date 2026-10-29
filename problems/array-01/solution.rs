pub fn zip_arrays<A, B, const N: usize>(a: [A; N], b: [B; N]) -> [(A, B); N] {
    let mut b = b.into_iter();
    a.map(|x| (x, b.next().unwrap()))
}
