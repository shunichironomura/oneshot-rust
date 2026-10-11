use std::ops::Range;

static mut NEXT: u64 = 0;

pub fn next_id() -> u64 {
    unsafe {
        let id = NEXT;
        NEXT = id + 1;
        id
    }
}

pub fn reserve(n: u64) -> Range<u64> {
    unsafe {
        let start = NEXT;
        NEXT = start + n;
        start..start + n
    }
}
