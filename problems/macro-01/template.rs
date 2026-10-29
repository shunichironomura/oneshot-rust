// The macro below is a placeholder that accepts any input. Replace its rules.
// The `pub(crate) use` line lets other modules call the macro by path.

// hashmap! { "a" => 1, "b" => 2 } builds a HashMap.
macro_rules! hashmap {
    ($($input:tt)*) => {
        todo!()
    };
}
pub(crate) use hashmap;
