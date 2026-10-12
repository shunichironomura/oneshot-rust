pub fn parse_orders(text: &str) -> Vec<(u32, &str)> {
    let mut orders = Vec::new();
    for line in text.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let [count, item] = words[..] else { continue };
        let Ok(count) = count.parse::<u32>() else { continue };
        if count == 0 {
            continue;
        }
        orders.push((count, item));
    }
    orders
}
