// count!(a + (b c)) is 3, as a usize constant.
macro_rules! count {
    () => {
        0usize
    };
    ($head:tt $($tail:tt)*) => {
        1usize + $crate::user::count!($($tail)*)
    };
}
pub(crate) use count;
