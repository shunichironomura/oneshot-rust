use std::fmt;

pub trait Outline: fmt::Display {
    /// Draws a frame of `*` around the `Display` text of `self`.
    fn outline(&self) -> String {
        let text = self.to_string();
        let width = text.chars().count();
        let border = "*".repeat(width + 4);
        let blank = format!("*{}*", " ".repeat(width + 2));
        format!("{border}\n{blank}\n* {text} *\n{blank}\n{border}")
    }
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Tags(pub Vec<String>);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Tags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl Outline for Point {}
impl Outline for Tags {}
