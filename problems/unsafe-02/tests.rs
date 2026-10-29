use crate::judge::{case, check_eq};
use crate::user::split_in_two;

pub fn run() {
    case("example-1", || {
        let mut v = [1, 2, 3, 4, 5];
        let (left, right) = split_in_two(&mut v, 2);
        check_eq(&*left, &[1, 2][..])?;
        check_eq(&*right, &[3, 4, 5][..])?;
        left[0] = 10;
        right[0] = 30;
        check_eq(v, [10, 2, 30, 4, 5])
    });
    case("case-1", || {
        let mut v = [1, 2, 3, 4];
        let (left, right) = split_in_two(&mut v, 2);
        for (l, r) in left.iter_mut().zip(right.iter_mut()) {
            std::mem::swap(l, r);
        }
        check_eq(v, [3, 4, 1, 2])
    });
    case("case-2", || {
        let mut v = [7, 8, 9];
        let (left, right) = split_in_two(&mut v, 0);
        check_eq(left.len(), 0)?;
        right[2] += left.len() as i32 + 1;
        check_eq(v, [7, 8, 10])
    });
    case("case-3", || {
        let mut v = [7, 8, 9];
        let (left, right) = split_in_two(&mut v, 3);
        check_eq(right.len(), 0)?;
        left[2] = 0;
        check_eq(v, [7, 8, 0])
    });
    case("case-4", || {
        let mut v: [i32; 0] = [];
        let (left, right) = split_in_two(&mut v, 0);
        check_eq((left.len(), right.len()), (0, 0))
    });
    case("case-5", || {
        let mut v = [1, 1, 1, 1, 1, 1];
        let (left, right) = split_in_two(&mut v, 4);
        right[1] = left.iter().sum();
        left[3] = right[1] * 2;
        check_eq(v, [1, 1, 1, 8, 1, 4])
    });
}
