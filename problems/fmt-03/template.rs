use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone)]
pub struct Credentials {
    pub user: String,
    pub password: String,
    pub scopes: Vec<String>,
}

#[derive(Clone)]
pub struct Inventory {
    pub items: BTreeMap<String, u32>,
    pub cache: Vec<u8>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Debug for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
