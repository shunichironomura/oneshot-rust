// matches_any!(value, PATTERN) is true if `value` matches the pattern.
// The pattern may be an or-pattern, as in matches_any!(x, 1 | 4..=6).
macro_rules! matches_any {
    ($value:expr, $p:pat) => {
        match $value {
            $p => true,
            _ => false,
        }
    };
}
pub(crate) use matches_any;

pub fn is_vowel(c: char) -> bool {
    matches_any!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

pub fn is_small(x: Option<u8>) -> bool {
    matches_any!(x, None | Some(0..=9))
}
