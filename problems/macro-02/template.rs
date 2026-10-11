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

// The macro below is a placeholder that accepts any input. Replace its rules.
// The `pub(crate) use` line lets other modules call the macro by path.

// json!({ "a": [1, null], "b": (x - 7) }) builds a Json value.
macro_rules! json {
    ($($input:tt)*) => {
        todo!()
    };
}
pub(crate) use json;
