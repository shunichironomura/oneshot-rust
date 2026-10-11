use crate::judge::{case, check_eq};
use crate::user::{hashtag_quote, json_snippet, number_pattern, png_magic, windows_path};

pub fn run() {
    case("sample-1", || {
        check_eq(number_pattern(), r"\d+\.\d+")?;
        check_eq(number_pattern().len(), 8)
    });
    case("sample-2", || {
        check_eq(windows_path(), r"C:\new\table\data.txt")?;
        check_eq(windows_path().len(), 21)
    });
    case("hidden-1", || {
        check_eq(json_snippet(), r#"{"name": "Ferris", "tags": ["crab"]}"#)?;
        check_eq(json_snippet().len(), 36)
    });
    case("hidden-2", || {
        check_eq(hashtag_quote(), r##"He said "#rust" twice"##)?;
        check_eq(hashtag_quote().len(), 21)
    });
    case("hidden-3", || {
        // The PNG signature: 0x89, "PNG", CR LF, 0x1A (Ctrl-Z), LF. Exactly 8 bytes.
        check_eq(png_magic(), [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A])?;
        check_eq(png_magic().len(), 8)
    });
}
