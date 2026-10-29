/// Appends `item` to `list` unless it is already there.
fn push_unique<'a>(list: &mut Vec<&'a str>, item: &'a str) {
    if !list.contains(&item) {
        list.push(item);
    }
}

pub fn merge(defaults: &[&'static str], extra: &str) -> Vec<String> {
    let mut all: Vec<&'static str> = defaults.to_vec();
    for part in extra.split(',') {
        let word = part.trim();
        if !word.is_empty() {
            push_unique(&mut all, word);
        }
    }
    all.iter().map(|word| word.to_string()).collect()
}
