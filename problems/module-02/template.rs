pub mod shapes {
    use crate::units::Cm;

    use checks::is_square;

    pub struct Rect {
        width: Cm,
        height: Cm,
    }

    impl Rect {
        pub(super) fn new(width: u32, height: u32) -> Rect {
            Rect { width: Cm(width), height: Cm(height) }
        }

        pub fn area(&self) -> u32 {
            self.width.0 * self.height.0
        }
    }

    mod checks {
        use super::Rect;
        use super::super::units::Cm;

        pub(super) fn is_square(rect: &Rect) -> bool {
            let Rect { width: Cm(w), height: Cm(h) } = rect;
            w == h
        }
    }
}

mod units {
    /// A length in whole centimeters.
    pub struct Cm(u32);
}

pub fn square(side: u32) -> shapes::Rect {
    shapes::Rect::new(side, side)
}

pub fn total_area(rects: &[shapes::Rect]) -> u32 {
    rects.iter().map(shapes::Rect::area).sum()
}
