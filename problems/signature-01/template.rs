pub fn largest(items: &[i32]) -> &i32 {
    let mut best = &items[0];
    for item in items {
        if item > best {
            best = item;
        }
    }
    best
}
