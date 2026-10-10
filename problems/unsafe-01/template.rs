pub fn push_and_sum(v: &mut Vec<i32>, extra: i32) -> i32 {
    let first: *const i32 = &v[0];
    v.push(extra);
    let total: i32 = v.iter().sum();
    total + unsafe { *first }
}
