pub fn split_in_two(v: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = v.len();
    assert!(mid <= len, "mid out of bounds");
    let ptr = v.as_mut_ptr();
    // SAFETY: both halves come from the same pointer, lie inside `v`, and do not overlap.
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
