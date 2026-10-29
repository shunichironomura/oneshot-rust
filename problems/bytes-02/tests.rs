use crate::judge::{case, check_eq};
use crate::user::{Kind, Packet, PacketError, decode_all};

const O: u8 = b'O';
const S: u8 = b'S';

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
    case("example-1", || {
        let stream = [
            O, S, 1, 1, 0, 0, 0, 0, 0, // Ping, seq 1, empty payload
            O, S, 2, 2, 0, 0, 0, 0, 1, b'!', // Data, seq 2, payload "!"
        ];
        check_eq(
            decode_all(&stream),
            Ok(vec![
                Packet { kind: Kind::Ping, seq: 1, payload: &[] },
                Packet { kind: Kind::Data, seq: 2, payload: b"!" },
            ]),
        )
    });
    case("example-2", || {
        let stream = [
            O, S, 1, 1, 0, 0, 0, 0, 0, // Ping, seq 1, empty payload
            O, S, 2, 2, 0, 0, 0, 0, 3, b'!', // Data, seq 2, 3 bytes declared, only 1 left
        ];
        check_eq(decode_all(&stream), Err(mismatch(3, 1)))?;
        check_eq(decode_all(&stream[..12]), Err(PacketError::TooShort))
    });
    case("case-1", || {
        check_eq(decode_all(&[]), Ok(vec![]))?;
        let mut stream = raw(3, 0x0102, 3, b"OS\x02");
        stream.extend(raw(1, 0, 0, &[]));
        stream.extend(raw(2, 0xFFFF, 9, &[0; 9]));
        stream.extend(raw(2, 0x8000, 1, &[0xEE]));
        check_eq(
            decode_all(&stream),
            Ok(vec![
                Packet { kind: Kind::Ack, seq: 0x0102, payload: b"OS\x02" },
                Packet { kind: Kind::Ping, seq: 0, payload: &[] },
                Packet { kind: Kind::Data, seq: 0xFFFF, payload: &[0; 9] },
                Packet { kind: Kind::Data, seq: 0x8000, payload: &[0xEE] },
            ]),
        )?;
        check_eq(
            decode_all(&raw(2, 5, 2, b"ok")),
            Ok(vec![Packet { kind: Kind::Data, seq: 5, payload: b"ok" }]),
        )
    });
    case("case-2", || {
        // Fewer than 9 bytes left where a packet should start: TooShort.
        let good = raw(1, 5, 3, b"abc");
        for n in [1, 2, 5, 8] {
            let mut stream = good.clone();
            stream.extend_from_slice(&good[..n]);
            check_eq(decode_all(&stream), Err(PacketError::TooShort))?;
            check_eq(decode_all(&good[..n]), Err(PacketError::TooShort))?;
        }
        // A declared length shorter than the rest just ends the packet early.
        check_eq(decode_all(&raw(1, 0, 1, b"abc")), Err(PacketError::TooShort))
    });
    case("case-3", || {
        // Not enough bytes for the payload: actual counts the bytes after the header.
        let good = raw(1, 5, 3, b"abc");
        let mut stream = good.clone();
        stream.extend_from_slice(&good[..11]);
        check_eq(decode_all(&stream), Err(mismatch(3, 2)))?;
        check_eq(decode_all(&raw(2, 0, 1, &[])), Err(mismatch(1, 0)))?;
        // The largest length must not overflow anything.
        let mut stream = good.clone();
        stream.extend(raw(2, 0, u32::MAX, &[1, 2, 3]));
        check_eq(decode_all(&stream), Err(mismatch(u32::MAX, 3)))
    });
    case("case-4", || {
        // The first invalid packet decides the error.
        let good = raw(1, 5, 3, b"abc");
        let mut stream = good.clone();
        stream.extend([b'X', S, 1, 0, 0, 0, 0, 0, 0]);
        stream.extend(raw(7, 0, 0, &[]));
        check_eq(decode_all(&stream), Err(PacketError::BadMagic))?;
        let mut stream = good.clone();
        stream.extend(raw(0, 0, 50, &[1]));
        stream.extend(&good[..3]);
        check_eq(decode_all(&stream), Err(PacketError::UnknownKind(0)))
    });
}
