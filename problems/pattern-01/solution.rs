pub fn filled_slots(slots: &[Option<u8>]) -> Vec<(usize, u8)> {
    let mut filled = Vec::new();
    for (i, slot) in slots.iter().enumerate() {
        if let Some(x) = slot {
            filled.push((i, *x));
        }
    }
    filled
}
