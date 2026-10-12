/// A regular expression that matches a decimal number such as `3.14`.
pub fn number_pattern() -> &'static str {
    r"\d+\.\d+"
}

/// A quote that mentions a hashtag.
pub fn hashtag_quote() -> &'static str {
    r##"He said "#rust" twice"##
}

/// The first 8 bytes of every PNG file.
pub fn png_magic() -> &'static [u8] {
    b"\x89PNG\r\n\x1a\n"
}
