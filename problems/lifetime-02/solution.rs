pub struct Tokenizer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Tokenizer<'a> {
        Tokenizer { input, pos: 0 }
    }

    pub fn next_token(&mut self) -> Option<&'a str> {
        let rest = &self.input[self.pos..];
        let start = rest.len() - rest.trim_start().len();
        let rest = &rest[start..];
        if rest.is_empty() {
            self.pos = self.input.len();
            return None;
        }
        let len = rest.find(char::is_whitespace).unwrap_or(rest.len());
        self.pos += start + len;
        Some(&rest[..len])
    }
}
