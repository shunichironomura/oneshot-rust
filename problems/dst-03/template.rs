use std::fmt::Display;

/// A packet: an id followed by its payload.
pub struct Packet<T> {
    pub id: u32,
    pub payload: T,
}

/// Puts a value that can be displayed into a new boxed packet.
pub fn label(id: u32, payload: String) -> Box<Packet<String>> {
    Box::new(Packet { id, payload })
}

/// Formats each packet as "#<id>: <payload>".
pub fn describe(ps: &[Box<Packet<String>>]) -> Vec<String> {
    ps.iter().map(|p| format!("#{}: {}", p.id, &p.payload)).collect()
}
