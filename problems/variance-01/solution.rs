/// Appends `item` to `list` unless it is already there.
fn push_unique<'a>(list: &mut Vec<&'a str>, item: &'a str) {
    if !list.contains(&item) {
        list.push(item);
    }
}

pub fn merge(defaults: &[&'static str], extra: &str) -> Vec<String> {
    // `Vec<&str>`, not `Vec<&'static str>`: the list also holds words borrowed from `extra`.
    let mut all: Vec<&str> = defaults.to_vec();
    for part in extra.split(',') {
        let word = part.trim();
        if !word.is_empty() {
            push_unique(&mut all, word);
        }
    }
    all.iter().map(|word| word.to_string()).collect()
}

/// Records `word` in `seen` if it is new.
fn add_word<'a>(seen: &mut Vec<&'a str>, word: &'a str) {
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
