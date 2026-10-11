/// A regular expression that matches a decimal number such as `3.14`.
pub fn number_pattern() -> &'static str {
    r"\d+\.\d+"
}

/// Where the report is saved on Windows.
pub fn windows_path() -> &'static str {
    r"C:\new\table\data.txt"
}

/// A small JSON object.
pub fn json_snippet() -> &'static str {
    r#"{"name": "Ferris", "tags": ["crab"]}"#
}

/// A quote that mentions a hashtag.
pub fn hashtag_quote() -> &'static str {
    r##"He said "#rust" twice"##
}

/// The first 8 bytes of every PNG file.
pub fn png_magic() -> &'static [u8] {
    b"\x89PNG\r\n\x1a\n"
}
