#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    /// `bytes[at..at + len]` is an invalid sequence.
    Invalid { at: usize, len: usize },
    /// `bytes[at..]` is an incomplete character at the end of the input.
    Incomplete { at: usize },
}

pub fn decode(bytes: &[u8]) -> Result<&str, DecodeError> {
    todo!()
}

pub fn decode_lossy(bytes: &[u8]) -> (String, usize) {
    todo!()
}

/// Decodes bytes that arrive in chunks.
#[derive(Debug, Default)]
pub struct Utf8Stream {
    /// Bytes that were received but not decoded yet.
    pending: Vec<u8>,
}

impl Utf8Stream {
    pub fn new() -> Self {
        todo!()
    }

    pub fn push(&mut self, chunk: &[u8]) -> String {
        todo!()
    }

    pub fn finish(self) -> String {
        todo!()
    }
}
