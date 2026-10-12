use crate::judge::{case, check_eq};
use crate::user::{Kind, Packet, PacketError};

const O: u8 = b'O';
const S: u8 = b'S';

fn decode(bytes: &[u8]) -> Result<Packet<'_>, PacketError> {
    Packet::try_from(bytes)
}

/// Raw bytes of one packet, built by hand: `declared` need not match `payload`.
fn raw(kind: u8, seq: u16, declared: u32, payload: &[u8]) -> Vec<u8> {
    let mut bytes = vec![O, S, kind];
    bytes.extend_from_slice(&seq.to_le_bytes());
    bytes.extend_from_slice(&declared.to_be_bytes());
    bytes.extend_from_slice(payload);
    bytes
}

fn mismatch(declared: u32, actual: usize) -> PacketError {
    PacketError::LengthMismatch { declared, actual }
}

pub fn run() {
    case("sample-1", || {
        let bytes = [O, S, 2, 0x34, 0x12, 0, 0, 0, 2, 0xAA, 0xBB];
        check_eq(
            Packet::try_from(&bytes[..]),
            Ok(Packet { kind: Kind::Data, seq: 0x1234, payload: &[0xAA, 0xBB] }),
        )?;
        check_eq(Packet::try_from(&bytes[..10]), Err(mismatch(2, 1)))?;
        check_eq(Packet::try_from(&bytes[..5]), Err(PacketError::TooShort))
    });
    case("sample-2", || {
        let packet = Packet { kind: Kind::Ack, seq: 7, payload: b"hi" };
        check_eq(packet.encode(), vec![O, S, 3, 7, 0, 0, 0, 0, 2, b'h', b'i'])
    });
    case("hidden-1", || {
        // seq is little-endian, the length big-endian.
        check_eq(
            decode(&[O, S, 1, 0xFF, 0x00, 0, 0, 0, 0]),
            Ok(Packet { kind: Kind::Ping, seq: 0x00FF, payload: &[] }),
        )?;
        check_eq(
            decode(&[O, S, 3, 0x00, 0xFF, 0, 0, 0, 1, 42]),
            Ok(Packet { kind: Kind::Ack, seq: 0xFF00, payload: &[42] }),
        )?;
        let mut bytes = [0x5A; 9 + 256];
        bytes[..9].copy_from_slice(&[O, S, 2, 0xFF, 0xFF, 0, 0, 1, 0]);
        check_eq(
            decode(&bytes),
            Ok(Packet { kind: Kind::Data, seq: u16::MAX, payload: &bytes[9..] }),
        )?;
        let via_into: Result<Packet<'_>, PacketError> = bytes.as_slice().try_into();
        check_eq(via_into.map(|p| p.payload.len()), Ok(256))
    });
    case("hidden-2", || {
        // Fewer than 9 bytes is TooShort, whatever the bytes are.
        let header = [O, S, 1, 0, 0, 0, 0, 0, 0];
        for n in 0..9 {
            check_eq(decode(&header[..n]), Err(PacketError::TooShort))?;
        }
        check_eq(decode(b"XY"), Err(PacketError::TooShort))?;
        check_eq(decode(&[0xFF; 8]), Err(PacketError::TooShort))?;
        check_eq(decode(&[O, S, 9, 0, 0, 0, 0, 9]), Err(PacketError::TooShort))
    });
    case("hidden-3", || {
        // The magic is checked before the kind, and the kind before the length.
        check_eq(decode(&[b'o', b's', 1, 0, 0, 0, 0, 0, 0]), Err(PacketError::BadMagic))?;
        check_eq(decode(&[S, O, 1, 0, 0, 0, 0, 0, 0]), Err(PacketError::BadMagic))?;
        check_eq(decode(&[O, b'X', 9, 0, 0, 0, 0, 0, 5]), Err(PacketError::BadMagic))?;
        check_eq(decode(&[b'X', S, 0, 0, 0, 0, 0, 0, 0, 1]), Err(PacketError::BadMagic))?;
        check_eq(decode(&[O, S, 0, 0, 0, 0, 0, 0, 0]), Err(PacketError::UnknownKind(0)))?;
        check_eq(decode(&[O, S, 4, 1, 2, 0, 0, 0, 7, 1]), Err(PacketError::UnknownKind(4)))?;
        check_eq(
            decode(&[O, S, 0xFF, 1, 2, 0xFF, 0xFF, 0xFF, 0xFF]),
            Err(PacketError::UnknownKind(0xFF)),
        )
    });
    case("hidden-4", || {
        // The payload must be exactly as long as declared.
        check_eq(decode(&raw(2, 0, 1, &[1, 2, 3])), Err(mismatch(1, 3)))?;
        check_eq(decode(&raw(1, 0, 0, &[9])), Err(mismatch(0, 1)))?;
        check_eq(decode(&[O, S, 2, 0, 0, 0, 0, 1, 0, 7]), Err(mismatch(256, 1)))?;
        check_eq(decode(&[O, S, 2, 0, 0, 1, 0, 0, 0]), Err(mismatch(1 << 24, 0)))?;
        // The largest length must not overflow anything.
        check_eq(decode(&raw(3, 0, u32::MAX, &[1, 2])), Err(mismatch(u32::MAX, 2)))?;
        check_eq(decode(&raw(3, 0, u32::MAX - 8, &[])), Err(mismatch(u32::MAX - 8, 0)))
    });
    case("hidden-5", || {
        let empty = Packet { kind: Kind::Ping, seq: 0xABCD, payload: &[] };
        check_eq(empty.encode(), vec![O, S, 1, 0xCD, 0xAB, 0, 0, 0, 0])?;
        // 300 = 0x012C
        let mut payload = [0x5A; 300];
        payload[0] = 1;
        payload[299] = 2;
        let long = Packet { kind: Kind::Data, seq: 0x0102, payload: &payload };
        let bytes = long.encode();
        check_eq(&bytes[..9], &[O, S, 2, 0x02, 0x01, 0, 0, 0x01, 0x2C][..])?;
        check_eq(&bytes[9..], &payload[..])?;
        // encode and try_from are inverses.
        let packets = [
            empty,
            long,
            Packet { kind: Kind::Ack, seq: 0, payload: b"OS\x01" },
            Packet { kind: Kind::Data, seq: u16::MAX, payload: &[0; 9] },
        ];
        for p in packets {
            check_eq(decode(&p.encode()), Ok(p))?;
        }
        Ok(())
    });
}
