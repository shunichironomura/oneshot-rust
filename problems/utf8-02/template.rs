#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    /// `bytes[at..at + len]` is an invalid sequence.
    Invalid { at: usize, len: usize },
    /// `bytes[at..]` is an incomplete character at the end of the input.
    Incomplete { at: usize },
}

pub fn decode(bytes: &[u8]) -> Result<&str, DecodeError> {
    // `from_utf8` checks the bytes and returns a `&str` that points into them: nothing is copied.
    std::str::from_utf8(bytes).map_err(|e| {
        let at = e.valid_up_to();
        match e.error_len() {
            Some(len) => DecodeError::Invalid { at, len },
            // `None`: the input ended in the middle of a character.
            None => DecodeError::Incomplete { at },
        }
    })
}

pub fn decode_lossy(bytes: &[u8]) -> (String, usize) {
    let mut text = String::with_capacity(bytes.len());
    let mut replaced = 0;
    // Each chunk is a valid part followed by one invalid sequence (empty in the last chunk).
    for chunk in bytes.utf8_chunks() {
        text.push_str(chunk.valid());
        if !chunk.invalid().is_empty() {
            text.push('?');
            replaced += 1;
        }
    }
    (text, replaced)
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
