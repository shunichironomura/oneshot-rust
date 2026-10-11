pub fn insert_sorted<T, F: FnMut(&T, &T) -> bool>(v: &mut Vec<T>, x: T, mut less: F) {
    // Phase 1: only look. `v` is not changed, so if `less` panics here, `v` is exactly as it
    // was, and `x`, which this function still owns, is dropped while unwinding.
    let pos = v.partition_point(|e| !less(&x, e));
    // Phase 2: change `v`. No code of the caller runs any more.
    v.insert(pos, x);
}
