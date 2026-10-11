pub fn bucket_by_initial(words: &[&str]) -> [Vec<String>; 26] {
    // Arrays implement `Default` up to length 32: this gives 26 separate empty `Vec`s.
    let mut buckets: [Vec<String>; 26] = Default::default();
    for word in words {
        if let Some(first) = word.chars().next() {
            if first.is_ascii_alphabetic() {
                // `char` has no arithmetic. `first` is ASCII here, so `as u8` loses nothing.
                let index = (first.to_ascii_lowercase() as u8 - b'a') as usize;
                buckets[index].push(word.to_string());
            }
        }
    }
    buckets
}

pub fn letters_used(buckets: &[Vec<String>; 26]) -> String {
    let mut letters = String::new();
    for letter in b'a'..=b'z' {
        // An index must be a `usize`, and a `u8` must be converted to a `char`.
        if !buckets[usize::from(letter - b'a')].is_empty() {
            letters.push(char::from(letter));
        }
    }
    letters
}
