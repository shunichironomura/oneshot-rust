use crate::judge::{case, check_eq};
use crate::user::to_levels;

pub fn run() {
    case("example-1", || {
        check_eq(
            to_levels(&[0.0, 0.5, 1.0, 1.5, -0.2, f32::NAN, f32::INFINITY]),
            vec![0, 128, 255, 255, 0, 0, 255],
        )
    });
    case("case-1", || {
        // Rounded to the nearest level, not truncated: 0.51 -> 1, 0.255 -> 0, 254.49 -> 254.
        // 0.3 gives exactly 76.5 in f32, and halves round away from zero (not to even): 77.
        check_eq(
            to_levels(&[0.002, 0.001, 0.998, 0.25, 0.75, 0.3]),
            vec![1, 0, 254, 64, 191, 77],
        )
    });
    case("case-2", || check_eq(to_levels(&[]), Vec::new()));
    case("case-3", || {
        // Above the range, including a level that rounds to 256 and an infinite product.
        check_eq(to_levels(&[1.001, 1.002, 2.0, 1e30, f32::MAX]), vec![255; 5])
    });
    case("case-4", || {
        // Below the range, including -0.0 and a level that rounds to -1.
        check_eq(
            to_levels(&[-0.0, -0.001, -0.002, -1.0, f32::MIN, f32::NEG_INFINITY]),
            vec![0; 6],
        )
    });
    case("case-5", || {
        let other_nan = f32::from_bits(0x7f80_0001);
        check_eq(
            to_levels(&[f32::NAN, 0.5, -f32::NAN, 1.0, other_nan]),
            vec![0, 128, 0, 255, 0],
        )
    });
    case("case-6", || {
        check_eq(
            to_levels(&[0.1, 0.2, 0.4, 0.6, 0.8, 0.05, 0.95, 0.123, 0.333, 0.667]),
            vec![26, 51, 102, 153, 204, 13, 242, 31, 85, 170],
        )
    });
}
