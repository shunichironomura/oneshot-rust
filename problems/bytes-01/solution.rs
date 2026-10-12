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
        match b {
            1 => Ok(Kind::Ping),
            2 => Ok(Kind::Data),
            3 => Ok(Kind::Ack),
            other => Err(PacketError::UnknownKind(other)),
        }
    }
}

const MAGIC: [u8; 2] = *b"OS";
const HEADER_LEN: usize = 9;

impl<'a> TryFrom<&'a [u8]> for Packet<'a> {
    type Error = PacketError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let (header, payload) = bytes
            .split_first_chunk::<HEADER_LEN>()
            .ok_or(PacketError::TooShort)?;
        let [m0, m1, kind, s0, s1, l0, l1, l2, l3] = *header;
        if [m0, m1] != MAGIC {
            return Err(PacketError::BadMagic);
        }
        let kind = Kind::try_from(kind)?;
        let declared = u32::from_be_bytes([l0, l1, l2, l3]);
        // Widen the u32 to usize: `payload.len() as u32` could cut off a long length.
        if payload.len() != declared as usize {
            return Err(PacketError::LengthMismatch { declared, actual: payload.len() });
        }
        Ok(Packet { kind, seq: u16::from_le_bytes([s0, s1]), payload })
    }
}

impl Packet<'_> {
    pub fn encode(&self) -> Vec<u8> {
        let len = self.payload.len() as u32;
        let mut out = Vec::with_capacity(HEADER_LEN + self.payload.len());
        out.extend_from_slice(&MAGIC);
        out.push(self.kind as u8);
        out.extend_from_slice(&self.seq.to_le_bytes());
        out.extend_from_slice(&len.to_be_bytes());
        out.extend_from_slice(self.payload);
        out
    }
}
