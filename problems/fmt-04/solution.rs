use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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
