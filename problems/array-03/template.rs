pub fn letters_used(buckets: &[Vec<String>; 26]) -> String {
    let mut letters = String::new();
    for letter in b'a'..=b'z' {
        if !buckets[letter - b'a'].is_empty() {
            letters.push(letter);
        }
    }
    letters
}
