/// A packet: an id followed by its payload. The payload may be unsized, such as `[u8]`, which is
/// allowed because it is the last field.
pub struct Packet<T: ?Sized> {
    pub id: u32,
    pub payload: T,
}

/// Puts the bytes into a new boxed packet. The `Box<Packet<[u8; N]>>` built by the body is
/// coerced to `Box<Packet<[u8]>>` when it is returned, so the length becomes run-time data.
pub fn make<const N: usize>(id: u32, bytes: [u8; N]) -> Box<Packet<[u8]>> {
    Box::new(Packet { id, payload: bytes })
}

/// The id plus the sum of the payload bytes.
pub fn checksum(p: &Packet<[u8]>) -> u32 {
    p.id + p.payload.iter().map(|&b| u32::from(b)).sum::<u32>()
}
