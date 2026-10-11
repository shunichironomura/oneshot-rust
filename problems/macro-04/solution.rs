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

// index_of!(value; P0, P1, ...) is Some(i) for the first pattern Pi that `value`
// matches, or None if it matches none of them.
macro_rules! index_of {
    // Internal rules: `$v` holds the value and `$i` is the index of the next pattern.
    (@check $v:ident, $i:expr;) => {
        None
    };
    (@check $v:ident, $i:expr; $p:pat $(, $($rest:tt)*)?) => {
        if matches!($v, $p) {
            Some($i)
        } else {
            $crate::user::index_of!(@check $v, $i + 1; $($($rest)*)?)
        }
    };
    ($value:expr; $($patterns:tt)*) => {
        match $value {
            value => $crate::user::index_of!(@check value, 0usize; $($patterns)*),
        }
    };
}
pub(crate) use index_of;

pub fn is_vowel(c: char) -> bool {
    matches_any!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

pub fn is_small(x: Option<u8>) -> bool {
    matches_any!(x, None | Some(0..=9))
}

/// 0 for an ASCII letter, 1 for an ASCII digit, 2 for a space, a tab or a newline.
pub fn char_class(c: char) -> Option<usize> {
    index_of!(c; 'a'..='z' | 'A'..='Z', '0'..='9', ' ' | '\t' | '\n')
}
