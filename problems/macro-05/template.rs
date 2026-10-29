// The macro below is a placeholder that accepts any input. Replace its rules.
// The `pub(crate) use` line lets other modules call the macro by path.

// max_of!(3, 9, 4) is 9.
macro_rules! max_of {
    ($($input:tt)*) => {
        todo!()
    };
}
pub(crate) use max_of;
