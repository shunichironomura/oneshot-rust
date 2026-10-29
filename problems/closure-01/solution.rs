pub fn make_counter(step: u32) -> impl FnMut() -> u32 {
    let mut current = 0;
    move || {
        current += step;
        current
    }
}
