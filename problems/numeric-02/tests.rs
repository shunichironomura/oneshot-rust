use crate::judge::{case, check_eq};
use crate::user::{
    bucket, charge, distance, middle, page_count, sum_checked, sum_wrapping, wrap_index,
};

pub fn run() {
    case("sample-1", || {
        check_eq(sum_wrapping(&[200, 100]), 44)?;
        check_eq(sum_checked(&[200, 100]), None)?;
        check_eq(sum_checked(&[100, 155]), Some(255))?;
        check_eq(charge(50, 20), 70)?;
        check_eq(charge(90, 200), 100)
    });
    case("sample-2", || {
        check_eq(wrap_index(-1, 5), 4)?;
        check_eq(wrap_index(12, 5), 2)?;
        check_eq(bucket(-1, 10), -1)?;
        check_eq(bucket(-11, 10), -2)?;
        check_eq(page_count(10, 3), 4)
    });
    case("sample-3", || {
        check_eq(middle(u64::MAX, u64::MAX - 2), u64::MAX - 1)?;
        check_eq(distance(i64::MIN, i64::MAX), u64::MAX)
    });
    case("hidden-1", || {
        // Empty slices, and long sums that would overflow a `u16` as well.
        check_eq(sum_wrapping(&[]), 0)?;
        check_eq(sum_wrapping(&[1, 2, 3]), 6)?;
        check_eq(sum_wrapping(&[255, 255, 255]), 253)?;
        check_eq(sum_wrapping(&[255; 300]), 212)?;
        check_eq(sum_checked(&[]), Some(0))?;
        check_eq(sum_checked(&[255]), Some(255))?;
        check_eq(sum_checked(&[100, 156]), None)?;
        check_eq(sum_checked(&[0, 0, 7]), Some(7))?;
        check_eq(sum_checked(&[255; 300]), None)
    });
    case("hidden-2", || {
        // The cap applies to every sum above 100, not only to sums above 255.
        check_eq(charge(0, 0), 0)?;
        check_eq(charge(0, 100), 100)?;
        check_eq(charge(99, 2), 100)?;
        check_eq(charge(50, 60), 100)?;
        check_eq(charge(100, 0), 100)?;
        check_eq(charge(100, 255), 100)?;
        check_eq(charge(1, 255), 100)
    });
    case("hidden-3", || {
        // Negative positions, multiples of `len`, and the extreme `i64` values.
        check_eq(wrap_index(-10, 5), 0)?;
        check_eq(wrap_index(-11, 5), 4)?;
        check_eq(wrap_index(4, 5), 4)?;
        check_eq(wrap_index(5, 5), 0)?;
        check_eq(wrap_index(0, 1), 0)?;
        check_eq(wrap_index(-1, 1_000_000), 999_999)?;
        check_eq(wrap_index(i64::MIN, 7), 6)?;
        check_eq(wrap_index(i64::MAX, 7), 0)?;
        check_eq(wrap_index(i64::MAX, 1), 0)?;
        check_eq(wrap_index(i64::MIN, 1_000_000), 224_192)?;
        check_eq(wrap_index(i64::MAX, 1_000_000), 775_807)
    });
    case("hidden-4", || {
        // Bucket boundaries, and `x` near `i32::MIN`.
        check_eq(bucket(0, 10), 0)?;
        check_eq(bucket(9, 10), 0)?;
        check_eq(bucket(10, 10), 1)?;
        check_eq(bucket(15, 10), 1)?;
        check_eq(bucket(-10, 10), -1)?;
        check_eq(bucket(-9, 1), -9)?;
        check_eq(bucket(i32::MIN, 1), i32::MIN)?;
        check_eq(bucket(i32::MIN, 10), -214_748_365)?;
        check_eq(bucket(i32::MIN, i32::MAX), -2)?;
        check_eq(bucket(-1, i32::MAX), -1)?;
        check_eq(bucket(i32::MAX, 2), 1_073_741_823)?;
        check_eq(bucket(i32::MAX, i32::MAX), 1)
    });
    case("hidden-5", || {
        // No items need no pages; large counts and page sizes must not overflow.
        check_eq(page_count(0, 3), 0)?;
        check_eq(page_count(9, 3), 3)?;
        check_eq(page_count(1, 1), 1)?;
        check_eq(page_count(0, u64::MAX), 0)?;
        check_eq(page_count(5, u64::MAX), 1)?;
        check_eq(page_count(u64::MAX, 1), u64::MAX)?;
        check_eq(page_count(u64::MAX, 2), 1u64 << 63)?;
        check_eq(page_count(u64::MAX, u64::MAX), 1)
    });
    case("hidden-6", || {
        // Rounding down, either argument order, and the full `u64` range.
        check_eq(middle(0, 0), 0)?;
        check_eq(middle(1, 1), 1)?;
        check_eq(middle(1, 2), 1)?;
        check_eq(middle(2, 1), 1)?;
        check_eq(middle(7, 2), 4)?;
        check_eq(middle(0, u64::MAX), u64::MAX / 2)?;
        check_eq(middle(u64::MAX, 1), 1u64 << 63)?;
        check_eq(middle(u64::MAX, u64::MAX), u64::MAX)
    });
    case("hidden-7", || {
        // Either argument order, and differences that do not fit in an `i64`.
        check_eq(distance(5, 5), 0)?;
        check_eq(distance(-3, 4), 7)?;
        check_eq(distance(4, -3), 7)?;
        check_eq(distance(i64::MIN, i64::MIN), 0)?;
        check_eq(distance(i64::MIN, 0), 1u64 << 63)?;
        check_eq(distance(0, i64::MIN), 1u64 << 63)?;
        check_eq(distance(-1, i64::MAX), 1u64 << 63)?;
        check_eq(distance(i64::MAX, i64::MIN), u64::MAX)
    });
}
