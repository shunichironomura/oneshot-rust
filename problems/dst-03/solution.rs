use std::fmt::Display;

/// A packet: an id followed by its payload. The payload may be unsized, such as `dyn Display`,
/// which is allowed because it is the last field.
pub struct Packet<T: ?Sized> {
    pub id: u32,
    pub payload: T,
}

/// Puts a value that can be displayed into a new boxed packet.
pub fn label<T: Display + 'static>(id: u32, payload: T) -> Box<Packet<dyn Display>> {
    Box::new(Packet { id, payload })
}

/// Formats each packet as "#<id>: <payload>".
pub fn describe(ps: &[Box<Packet<dyn Display>>]) -> Vec<String> {
    ps.iter().map(|p| format!("#{}: {}", p.id, &p.payload)).collect()
}
