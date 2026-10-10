pub fn push_doubled(v: &mut Vec<i32>) {
    let doubled: Vec<i32> = v.iter().filter(|&&x| x > 0).map(|&x| x * 2).collect();
    v.extend(doubled);
}
