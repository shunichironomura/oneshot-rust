use crate::judge::{case, check};
use crate::user::moving_average;

fn close(got: Vec<f64>, expected: &[f64]) -> Result<(), String> {
    let ok = got.len() == expected.len()
        && got.iter().zip(expected).all(|(g, e)| (g - e).abs() < 1e-9);
    check(ok, &format!("got {got:?}, expected {expected:?}"))
}

pub fn run() {
    case("example-1", || {
        close(moving_average(&[1, 2, 3, 4, 5], 2), &[1.5, 2.5, 3.5, 4.5])
    });
    case("example-2", || close(moving_average(&[1, 2], 3), &[]));
    case("case-1", || close(moving_average(&[4, -2, 7], 1), &[4.0, -2.0, 7.0]));
    case("case-2", || close(moving_average(&[1, 2, 4], 3), &[7.0 / 3.0]));
    case("case-3", || {
        close(moving_average(&[1, 2, 3], 0), &[])?;
        close(moving_average(&[], 1), &[])
    });
    case("case-4", || {
        close(
            moving_average(&[i32::MAX, i32::MAX, i32::MIN, i32::MIN], 2),
            &[2147483647.0, -0.5, -2147483648.0],
        )
    });
    case("case-5", || close(moving_average(&[-3, -4, 10, 0], 3), &[1.0, 2.0]));
}
