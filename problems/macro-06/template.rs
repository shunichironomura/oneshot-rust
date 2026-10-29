// The macro below is a placeholder that accepts any input. Replace its rules.
// The `pub(crate) use` line lets other modules call the macro by path.

// count!(a + (b c)) is 3, as a usize constant.
macro_rules! count {
    ($($input:tt)*) => {
        0usize
    };
}
pub(crate) use count;
