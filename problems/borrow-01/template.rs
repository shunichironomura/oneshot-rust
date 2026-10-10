pub fn push_doubled(v: &mut Vec<i32>) {
    for x in v.iter() {
        if *x > 0 {
            v.push(*x * 2);
        }
    }
}
