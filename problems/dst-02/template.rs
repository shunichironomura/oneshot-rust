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
