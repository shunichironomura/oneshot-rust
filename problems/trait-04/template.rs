/// Turns a value into a short, readable text.
pub trait Describe {
    fn describe(&self) -> String;
}

impl Describe for i32 {
    fn describe(&self) -> String {
        self.to_string()
    }
}

impl Describe for bool {
    fn describe(&self) -> String {
        String::from(if *self { "yes" } else { "no" })
    }
}

impl Describe for str {
    fn describe(&self) -> String {
        format!("{self:?}")
    }
}

impl Describe for String {
    fn describe(&self) -> String {
        self.as_str().describe()
    }
}

impl Describe for [i32] {
    fn describe(&self) -> String {
        let parts: Vec<String> = self.iter().map(|item| item.describe()).collect();
        format!("[{}]", parts.join(", "))
    }
}

impl Describe for Vec<i32> {
    fn describe(&self) -> String {
        self.as_slice().describe()
    }
}

impl Describe for Option<i32> {
    fn describe(&self) -> String {
        match self {
            Some(value) => format!("some({})", value.describe()),
            None => String::from("none"),
        }
    }
}

impl Describe for (i32, i32) {
    fn describe(&self) -> String {
        format!("({}, {})", self.0.describe(), self.1.describe())
    }
}

impl<T: Describe> Describe for &T {
    fn describe(&self) -> String {
        (**self).describe()
    }
}
