/// The sum of the scores. A name longer than 3 bytes earns 5 extra points.
pub fn bonus_total(scores: &[(String, u32)]) -> u32 {
    let mut total = 0;
    for &(ref name, mut s) in scores {
        if name.len() > 3 {
            s += 5;
        }
        total += s;
    }
    total
}
