pub fn split_in_two(v: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = v.len();
    let ptr = v.as_mut_ptr();
    let whole: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    let left = &mut v[..mid];
    let right = &mut whole[mid..];
    (left, right)
}
