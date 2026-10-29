pub fn filled_slots(slots: &[Option<u8>]) -> Vec<(usize, u8)> {
    let mut filled = Vec::new();
    for (i, Some(x)) in slots.iter().enumerate() {
        filled.push((i, *x));
    }
    filled
}
