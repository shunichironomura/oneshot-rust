Rust 2024 reserves `gen` as a keyword, for `gen` blocks (a planned way to write iterators); the
`rand` crate renamed its `Rng::gen` to `Rng::random` for this reason. In 2024 code the word `gen`
can no longer be an identifier, so both the definition `pub fn gen` and the call `self.gen()` in
`gen_below` are syntax errors. Renaming is not an option because the harness uses the name, so write
it as the raw identifier `r#gen` in both places. `r#gen` is the same name as `gen`: a 2021 crate
still calls this method as `.gen()`, while 2024 code such as the harness writes `rng.r#gen()`. Only
the exact word is reserved, so `gen_below` and the string `"gen"` need no change.

`type` has been a keyword in every edition, which is why the 2021 code already wrote the field as
`r#type`. A raw identifier works for local variables and in the field init shorthand
`Token { r#type, text: ... }` too, so `classify` compiles as it is.
