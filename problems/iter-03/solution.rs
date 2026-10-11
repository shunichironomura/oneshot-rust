use std::iter;

pub fn last_long(words: &[&str], min: usize) -> Option<usize> {
    // `rposition` searches from the back but counts the index from the front.
    words.iter().rposition(|w| w.chars().count() >= min)
}

pub fn split_header<'a>(lines: &[&'a str]) -> (Vec<&'a str>, Vec<&'a str>) {
    let n = lines
        .iter()
        .position(|line| !line.starts_with('#'))
        .unwrap_or(lines.len());
    let (header, body) = lines.split_at(n);
    (header.to_vec(), body.to_vec())
}

pub fn running_balance(start: i64, txns: &[i64]) -> Vec<i64> {
    txns.iter()
        .scan(start, |balance, &t| {
            let next = *balance + t;
            if next < 0 {
                return None; // ends the iteration
            }
            *balance = next;
            Some(next)
        })
        .collect()
}

pub fn powers_of(base: u64) -> Vec<u64> {
    iter::successors(Some(1u64), |&p| p.checked_mul(base)).collect()
}
