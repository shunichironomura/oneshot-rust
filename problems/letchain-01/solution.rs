#[derive(Debug, PartialEq)]
pub enum Token {
    Tiny(u64),
    Small(u64),
    Large(u64),
    Hex(u32),
    Pair(String, String),
    Word(String),
}

pub fn classify(tok: &str) -> Token {
    if let Ok(n) = tok.parse::<u64>() {
        return match n {
            0..10 => Token::Tiny(n),
            10..1000 => Token::Small(n),
            1000.. => Token::Large(n),
        };
    }
    // `from_str_radix` alone would also accept "+12345", so check every digit first.
    if let Some(digits) = tok.strip_prefix('#')
        && digits.len() == 6
        && digits.bytes().all(|b| b.is_ascii_hexdigit())
        && let Ok(value) = u32::from_str_radix(digits, 16)
    {
        return Token::Hex(value);
    }
    if let Some((before, after)) = tok.split_once(':')
        && !before.is_empty()
        && !after.is_empty()
    {
        return Token::Pair(before.to_string(), after.to_string());
    }
    Token::Word(tok.to_string())
}

pub fn sum_first_two(nums: &[u8]) -> Option<u8> {
    match nums {
        [a, b, ..] => a.checked_add(*b),
        _ => None,
    }
}
