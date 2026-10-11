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
        f.debug_struct("Credentials")
            .field("user", &self.user)
            // The `Debug` of `format_args!` writes the text as is. A `&str` would get quotes.
            .field("password", &format_args!("<redacted>"))
            .field("scopes", &self.scopes)
            .finish()
    }
}

impl fmt::Debug for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `finish_non_exhaustive` closes the struct with `..`.
        f.debug_struct("Inventory")
            .field("items", &self.items)
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let opaque = self.a == 255;
        // Keep the builder itself in a variable: `field` returns a `&mut` to it, which
        // must not outlive the builder.
        let mut tuple = f.debug_tuple(if opaque { "Rgb" } else { "Rgba" });
        tuple.field(&self.r).field(&self.g).field(&self.b);
        if !opaque {
            tuple.field(&self.a);
        }
        tuple.finish()
    }
}
