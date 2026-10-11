// hashmap! { "a" => 1, "b" => 2 } builds a HashMap.
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ($($key:expr => $value:expr),+ $(,)?) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )+
        map
    }};
}
pub(crate) use hashmap;

// max_of!(3, 9, 4) is 9.
macro_rules! max_of {
    ($first:expr $(, $rest:expr)* $(,)?) => {{
        let max = $first;
        $(
            let next = $rest;
            let max = if next > max { next } else { max };
        )*
        max
    }};
}
pub(crate) use max_of;

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
