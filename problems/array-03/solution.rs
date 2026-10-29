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
