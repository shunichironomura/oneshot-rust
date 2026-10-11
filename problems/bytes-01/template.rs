#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Ping = 1,
    Data = 2,
    Ack = 3,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PacketError {
    TooShort,
    BadMagic,
    UnknownKind(u8),
    LengthMismatch { declared: u32, actual: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Packet<'a> {
    pub kind: Kind,
    pub seq: u16,
    pub payload: &'a [u8],
}

impl TryFrom<u8> for Kind {
    type Error = PacketError;

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl<'a> TryFrom<&'a [u8]> for Packet<'a> {
    type Error = PacketError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Packet<'_> {
    pub fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

pub fn decode_all(stream: &[u8]) -> Result<Vec<Packet<'_>>, PacketError> {
    todo!()
}
