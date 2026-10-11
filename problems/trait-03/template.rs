use std::fmt;

pub trait Outline {
    /// Draws a frame of `*` around the `Display` text of `self`.
    fn outline(&self) -> String {
        todo!()
    }
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Tags(pub Vec<String>);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Display for Tags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Outline for Point {}
impl Outline for Tags {}
