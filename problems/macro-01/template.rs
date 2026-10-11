// Each macro below is a placeholder that accepts any input. Replace its rules.
// The `pub(crate) use` lines let other modules call the macros by path.

// hashmap! { "a" => 1, "b" => 2 } builds a HashMap.
macro_rules! hashmap {
    ($($input:tt)*) => {
        todo!()
    };
}
pub(crate) use hashmap;

// max_of!(3, 9, 4) is 9.
macro_rules! max_of {
    ($($input:tt)*) => {
        todo!()
    };
}
pub(crate) use max_of;

// count!(a + (b c)) is 3, as a usize constant.
macro_rules! count {
    ($($input:tt)*) => {
        0usize
    };
}
pub(crate) use count;
