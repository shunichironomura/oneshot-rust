pub fn parse_hex_color(s: &str) -> Option<[u8; 3]> {
    // The value of every character after the '#', or None if any of them is not a hex digit.
    let digits: Vec<u8> = s
        .strip_prefix('#')?
        .chars()
        .map(|c| c.to_digit(16).map(|d| d as u8))
        .collect::<Option<_>>()?;
    match digits[..] {
        [r, g, b] => Some([r * 17, g * 17, b * 17]),
        [r1, r2, g1, g2, b1, b2] => Some([r1 * 16 + r2, g1 * 16 + g2, b1 * 16 + b2]),
        _ => None,
    }
}

pub fn to_hex(rgb: [u8; 3]) -> String {
    let [r, g, b] = rgb;
    // A channel has two equal digits `dd` exactly when it is a multiple of 17 (dd = 16 * d + d).
    if rgb.iter().all(|&c| c % 17 == 0) {
        format!("#{:x}{:x}{:x}", r / 17, g / 17, b / 17)
    } else {
        format!("#{r:02x}{g:02x}{b:02x}")
    }
}
