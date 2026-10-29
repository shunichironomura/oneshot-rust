use std::borrow::Cow;

pub fn normalize_ws(s: &str) -> Cow<'_, str> {
    let trimmed = s.trim_ascii();
    // A run needs replacing if it is longer than one character or is not a plain space.
    let needs_copy =
        trimmed.contains("  ") || trimmed.contains(|c: char| c.is_ascii_whitespace() && c != ' ');
    if needs_copy {
        Cow::Owned(trimmed.split_ascii_whitespace().collect::<Vec<_>>().join(" "))
    } else {
        Cow::Borrowed(trimmed)
    }
}
