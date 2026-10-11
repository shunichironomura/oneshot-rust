Build the core of `minigrep`, the small search tool from chapters 12 and 13 of the Rust Book. There
are no real files or environment variables here: the command-line arguments come in as an iterator
of `String`s, and the contents of the file come in as a string. The `Config` struct is already in
the editor.

```rust
#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config, &'static str>;
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>;
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>;
```

`Config::build` reads the arguments in order:

1. The first item is the program name and is ignored.
2. The second item is `query`. If there is no second item (this includes an empty iterator),
   return `Err("Didn't get a query string")`.
3. The third item is `file_path`. If there is none, return `Err("Didn't get a file path")`.
4. A fourth item is optional. If it is exactly `--ignore-case`, `ignore_case` is `true`; without a
   fourth item it is `false`. Any other fourth item, or any fifth item, gives
   `Err("Unexpected argument")`.

Only the position of an item matters. An empty string is still an item, and `--ignore-case` in any
other position is an ordinary value.

`search` returns the lines of `contents` that contain `query`, in their original order.

- Lines are split the way `str::lines` splits them: a line ends at `\n` or `\r\n`, the line ending is
  not part of the line, and a line ending at the very end of `contents` does not start another
  (empty) line. Empty `contents` has no lines.
- Matching is case-sensitive. A line is returned once, even if it contains `query` several times,
  and equal lines are each returned. Every line, including an empty one, contains the empty query.

`search_case_insensitive` works the same way, except that a line matches when `line.to_lowercase()`
contains `query.to_lowercase()` (lowercasing follows Unicode, so `É` becomes `é`). It still returns
the lines as they appear in `contents`.

The returned lines borrow only from `contents`: they stay usable after `query` is dropped.

### Examples

With the arguments `minigrep to poem.txt`, `Config::build` returns
`Ok(Config { query: "to", file_path: "poem.txt", ignore_case: false })` (with `String` fields).

```rust
let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
search("duct", contents)                  // ["safe, fast, productive."]
search_case_insensitive("rUsT", contents) // ["Rust:", "Trust me."]
```
