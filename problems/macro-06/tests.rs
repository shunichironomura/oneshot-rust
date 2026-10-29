use crate::judge::{case, check_eq};

// No `use` for the macro: it must work by path alone.

pub fn run() {
    case("example-1", || {
        const N: usize = crate::user::count!(a + (b c));
        const Z: usize = crate::user::count!();
        check_eq(N, 3)?;
        check_eq(Z, 0)
    });
    case("case-1", || {
        // Groups count as one token tree; punctuation, literals and lifetimes count too.
        const A: usize = crate::user::count!(fn main() { println!("hi"); });
        const B: usize = crate::user::count!(x :: y => 'a "s" 1.5 [1, 2, 3]);
        const C: usize = crate::user::count!(a, b, c);
        const D: usize = crate::user::count!((a, b));
        let array = [0u8; crate::user::count!(x y z)];
        check_eq((A, B, C, D), (4, 8, 5, 1))?;
        check_eq(array.len(), 3)
    });
}
