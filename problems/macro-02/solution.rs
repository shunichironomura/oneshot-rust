#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl From<bool> for Json {
    fn from(b: bool) -> Self {
        Json::Bool(b)
    }
}

impl From<i32> for Json {
    fn from(n: i32) -> Self {
        Json::Num(f64::from(n))
    }
}

impl From<f64> for Json {
    fn from(n: f64) -> Self {
        Json::Num(n)
    }
}

impl From<&str> for Json {
    fn from(s: &str) -> Self {
        Json::Str(s.to_string())
    }
}

impl From<String> for Json {
    fn from(s: String) -> Self {
        Json::Str(s)
    }
}

// json!({ "a": [1, null], "b": (x - 7) }) builds a Json value.
macro_rules! json {
    (null) => {
        $crate::user::Json::Null
    };
    // Every value is a single token tree, so `$element:tt` takes a whole nested
    // array or object, and the recursive call handles it.
    ([$($element:tt),* $(,)?]) => {
        $crate::user::Json::Arr(::std::vec![$($crate::user::json!($element)),*])
    };
    ({$($key:literal : $value:tt),* $(,)?}) => {
        $crate::user::Json::Obj(::std::vec![
            $((::std::string::String::from($key), $crate::user::json!($value))),*
        ])
    };
    // `true`, `false`, a number or a string literal, or an expression in parentheses.
    // This rule matches any token tree, so it must come after the other rules.
    ($other:tt) => {
        $crate::user::Json::from($other)
    };
}
pub(crate) use json;
