pub fn bucket_by_initial(words: &[&str]) -> [Vec<String>; 26] {
    let mut buckets: [Vec<String>; 26] = Default::default();
    for word in words {
        if let Some(first) = word.chars().next() {
            if first.is_ascii_alphabetic() {
                let index = (first.to_ascii_lowercase() - 'a') as usize;
                buckets[index].push(word.to_string());
            }
        }
    }
    buckets
}
