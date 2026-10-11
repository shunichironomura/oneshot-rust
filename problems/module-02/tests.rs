use crate::judge::{case, check, check_eq};
use crate::user::shapes::{Rect, is_square};
use crate::user::{square, total_area};

pub fn run() {
    case("sample-1", || {
        check_eq(Rect::new(2, 3).area(), 6)?;
        check_eq(square(4).area(), 16)
    });
    case("sample-2", || check_eq(total_area(&[Rect::new(1, 2), Rect::new(3, 3)]), 11));
    case("sample-3", || {
        check_eq(is_square(&square(2)), true)?;
        check_eq(is_square(&Rect::new(1, 2)), false)
    });
    case("hidden-1", || {
        check_eq(is_square(&Rect::new(3, 3)), true)?;
        check_eq(is_square(&Rect::new(3, 2)), false)?;
        check_eq(is_square(&Rect::new(2, 3)), false)?;
        check_eq(is_square(&square(9)), true)?;
        check_eq(Rect::new(7, 1).area(), 7)
    });
    case("hidden-2", || {
        // Zero lengths are allowed, and a 0 x 0 rectangle is a square.
        check_eq(Rect::new(0, 7).area(), 0)?;
        check_eq(square(0).area(), 0)?;
        check_eq(is_square(&Rect::new(0, 0)), true)?;
        check_eq(is_square(&Rect::new(0, 5)), false)?;
        check_eq(is_square(&Rect::new(5, 0)), false)
    });
    case("hidden-3", || {
        check_eq(total_area(&[]), 0)?;
        check_eq(total_area(&[square(5)]), 25)?;
        check_eq(total_area(&[square(1), Rect::new(2, 5), square(3), Rect::new(0, 9)]), 20)
    });
    case("hidden-4", || {
        // Large lengths; the areas still fit in a u32.
        check_eq(Rect::new(65535, 65535).area(), 4_294_836_225)?;
        check_eq(Rect::new(1, u32::MAX).area(), u32::MAX)?;
        check_eq(is_square(&square(u32::MAX)), true)?;
        check_eq(is_square(&Rect::new(u32::MAX, u32::MAX - 1)), false)
    });
    case("hidden-5", || {
        // Rectangles kept in a Vec: the whole of it, a part of it, and they stay usable.
        let rects: Vec<Rect> = (1..=4).map(square).collect();
        check_eq(total_area(&rects), 30)?;
        check_eq(total_area(&rects[1..3]), 13)?;
        check(rects.iter().all(is_square), "every rectangle from square() is a square")
    });
}
