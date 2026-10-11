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

const MAGIC: [u8; 2] = *b"OS";
const HEADER_LEN: usize = 9;

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

/// The header fields of one packet.
struct Header {
    kind: Kind,
    seq: u16,
    declared: u32,
}

/// Checks the header at the start of `bytes` (checks 1 to 3) and returns it together with
/// everything after it.
fn split_header(bytes: &[u8]) -> Result<(Header, &[u8]), PacketError> {
    let (header, rest) = bytes
        .split_first_chunk::<HEADER_LEN>()
        .ok_or(PacketError::TooShort)?;
    let [m0, m1, kind, s0, s1, l0, l1, l2, l3] = *header;
    if [m0, m1] != MAGIC {
        return Err(PacketError::BadMagic);
    }
    let header = Header {
        kind: Kind::try_from(kind)?,
        seq: u16::from_le_bytes([s0, s1]),
        declared: u32::from_be_bytes([l0, l1, l2, l3]),
    };
    Ok((header, rest))
}

impl<'a> TryFrom<&'a [u8]> for Packet<'a> {
    type Error = PacketError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let (Header { kind, seq, declared }, payload) = split_header(bytes)?;
        // Widen the u32 to usize: `payload.len() as u32` could cut off a long length.
        if payload.len() != declared as usize {
            return Err(PacketError::LengthMismatch { declared, actual: payload.len() });
        }
        Ok(Packet { kind, seq, payload })
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

pub fn decode_all(stream: &[u8]) -> Result<Vec<Packet<'_>>, PacketError> {
    let mut packets = Vec::new();
    let mut rest = stream;
    while !rest.is_empty() {
        let (Header { kind, seq, declared }, after_header) = split_header(rest)?;
        let (payload, next) = after_header
            .split_at_checked(declared as usize)
            .ok_or(PacketError::LengthMismatch { declared, actual: after_header.len() })?;
        packets.push(Packet { kind, seq, payload });
        rest = next;
    }
    Ok(packets)
}
