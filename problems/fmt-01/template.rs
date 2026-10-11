use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    pub cents: i64,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
