pub fn first_negative(v: &[i32]) -> Option<i32> {
    v.iter().copied().find(|&x| x < 0)
}

pub fn position_of_max(v: &[i32]) -> Option<usize> {
    let max = v.iter().max()?;
    v.iter().position(|x| x == max)
}

pub fn count_long(words: &[&str], min: usize) -> usize {
    words.iter().filter(|w| w.len() >= min).count()
}

pub fn contains_word(words: &[String], w: &str) -> bool {
    words.iter().any(|s| s == w)
}

pub fn total_len(words: &[String]) -> usize {
    words.iter().map(|w| w.len()).sum()
}
