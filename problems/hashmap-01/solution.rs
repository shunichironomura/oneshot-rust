use std::collections::HashMap;

pub fn word_count(text: &str) -> Vec<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for w in text.split_whitespace() {
        *counts.entry(w.to_lowercase()).or_insert(0) += 1;
    }
    let mut result: Vec<(String, usize)> = counts.into_iter().collect();
    result.sort_by(|(wa, na), (wb, nb)| nb.cmp(na).then_with(|| wa.cmp(wb)));
    result
}
