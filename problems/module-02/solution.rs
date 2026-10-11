pub mod shapes {
    // `super` is `crate::user`, where `units` is. `crate` would be the root, above `crate::user`.
    use super::units::Cm;

    // `checks` stays private: this re-export is the public path `shapes::is_square`.
    pub use checks::is_square;

    pub struct Rect {
        width: Cm,
        height: Cm,
    }

    impl Rect {
        // The harness is outside `crate::user`, so `pub(super)` is not enough.
        pub fn new(width: u32, height: u32) -> Rect {
            Rect { width: Cm(width), height: Cm(height) }
        }

        pub fn area(&self) -> u32 {
            self.width.0 * self.height.0
        }
    }

    mod checks {
        use super::Rect;
        // Here `super` is `shapes`, so go up two levels.
        use super::super::units::Cm;

        // `pub use` can re-export it only if the function itself is `pub`.
        pub fn is_square(rect: &Rect) -> bool {
            let Rect { width: Cm(w), height: Cm(h) } = rect;
            w == h
        }
    }
}

mod units {
    // `pub(super)` makes the field visible everywhere in `crate::user`, including `shapes` and
    // `shapes::checks`, but not outside it.
    /// A length in whole centimeters.
    pub struct Cm(pub(super) u32);
}

pub fn square(side: u32) -> shapes::Rect {
    shapes::Rect::new(side, side)
}

pub fn total_area(rects: &[shapes::Rect]) -> u32 {
    rects.iter().map(shapes::Rect::area).sum()
}
