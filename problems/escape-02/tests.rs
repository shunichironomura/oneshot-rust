use crate::judge::{case, check_eq};
use crate::user::png_magic;

pub fn run() {
    case("example-1", || {
        // The PNG signature: 0x89, "PNG", CR LF, 0x1A (Ctrl-Z), LF. Exactly 8 bytes.
        check_eq(png_magic(), [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A])?;
        check_eq(png_magic().len(), 8)
    });
}
