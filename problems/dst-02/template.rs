use std::fmt::Display;

/// A packet: an id followed by its payload.
pub struct Packet<T> {
    pub id: u32,
    pub payload: T,
}

/// Puts the bytes into a new boxed packet.
pub fn make(id: u32, bytes: Vec<u8>) -> Box<Packet<Vec<u8>>> {
    Box::new(Packet { id, payload: bytes })
}

/// The id plus the sum of the payload bytes.
pub fn checksum(p: &Packet<Vec<u8>>) -> u32 {
    p.id + p.payload.iter().map(|&b| u32::from(b)).sum::<u32>()
}

/// The id of the packet with the longest payload (the first one on ties).
pub fn longest(ps: &[Box<Packet<Vec<u8>>>]) -> Option<u32> {
    let mut best = ps.first()?;
    for p in ps {
        if p.payload.len() > best.payload.len() {
            best = p;
        }
    }
    Some(best.id)
}

/// Puts a value that can be displayed into a new boxed packet.
pub fn label(id: u32, payload: String) -> Box<Packet<String>> {
    Box::new(Packet { id, payload })
}

/// Formats each packet as "#<id>: <payload>".
pub fn describe(ps: &[Box<Packet<String>>]) -> Vec<String> {
    ps.iter().map(|p| format!("#{}: {}", p.id, &p.payload)).collect()
}
