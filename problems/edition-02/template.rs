/// Each entry is worth its points plus a bonus of 1.
pub fn score(points: Box<[u32]>) -> u32 {
    let mut total = 0;
    for p in points.into_iter() {
        total += *p;
    }
    total + points.len() as u32
}
