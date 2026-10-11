pub fn shout<const N: usize>(names: [String; N]) -> [String; N] {
    names.map(|name| name.to_uppercase() + "!")
}

pub fn transpose<const R: usize, const C: usize>(m: [[i32; C]; R]) -> [[i32; R]; C] {
    let mut out = [[0; R]; C];
    for (r, row) in m.into_iter().enumerate() {
        for (c, x) in row.into_iter().enumerate() {
            out[c][r] = x;
        }
    }
    out
}

pub fn zip_arrays<A, B, const N: usize>(a: [A; N], b: [B; N]) -> [(A, B); N] {
    let mut b = b.into_iter();
    a.map(|x| (x, b.next().unwrap()))
}
