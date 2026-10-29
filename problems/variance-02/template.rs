/// Records `word` in `seen` if it is new.
fn add_word<'a>(seen: &'a mut Vec<&'a str>, word: &'a str) {
    if seen.iter().all(|w| *w != word) {
        seen.push(word);
    }
}

pub fn unique_upper(text: &str) -> Vec<String> {
    let mut seen = Vec::new();
    for word in text.split_whitespace() {
        add_word(&mut seen, word);
    }
    seen.iter().map(|word| word.to_uppercase()).collect()
}
