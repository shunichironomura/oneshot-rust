use std::ptr;

pub fn insert_sorted<T, F: FnMut(&T, &T) -> bool>(v: &mut Vec<T>, x: T, mut less: F) {
    v.push(x);
    let len = v.len();
    let p = v.as_mut_ptr();
    unsafe {
        // Take the new element out of the last slot, then shift every element that must come
        // after it one slot to the right, and write it into the gap.
        let tmp = ptr::read(p.add(len - 1));
        let mut i = len - 1;
        while i > 0 && less(&tmp, &*p.add(i - 1)) {
            ptr::copy_nonoverlapping(p.add(i - 1), p.add(i), 1);
            i -= 1;
        }
        ptr::write(p.add(i), tmp);
    }
}
