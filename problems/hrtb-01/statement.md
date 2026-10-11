The body of `map_lines` is already written, but its signature is wrong and it does not compile.
Change **only the signature** so that the test harness (the `tests.rs` tab) can call it. Do not
change the body.

`map_lines` normalizes the line endings of `text` (`\r\n` becomes `\n`), passes each line to `f`,
and collects the string slices that `f` returns into owned `String`s.

The harness passes both closures such as `|s| s.trim()` and an ordinary function
`fn first_word(s: &str) -> &str`. In every case, `f` returns a part of the line it was given.
