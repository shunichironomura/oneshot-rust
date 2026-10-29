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
