pub fn bucket_by_initial(words: &[&str]) -> [Vec<String>; 26] {
    // A constant can be repeated even though `Vec` is not `Copy`.
    let mut buckets: [Vec<String>; 26] = [const { Vec::new() }; 26];
    for word in words {
        if let Some(first) = word.chars().next() {
            if first.is_ascii_alphabetic() {
                // `first` is ASCII here, so `as u8` loses nothing.
                let index = (first.to_ascii_lowercase() as u8 - b'a') as usize;
                buckets[index].push(word.to_string());
            }
        }
    }
    buckets
}

pub fn letters_used(buckets: &[Vec<String>; 26]) -> String {
    let mut letters = String::new();
    for (i, bucket) in buckets.iter().enumerate() {
        if !bucket.is_empty() {
            // Only a `u8` can be cast to `char`.
            letters.push((b'a' + i as u8) as char);
        }
    }
    letters
}
