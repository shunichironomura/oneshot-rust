pub fn parse_orders(text: &str) -> Vec<(u32, &str)> {
    let mut orders = Vec::new();
    for line in text.lines() {
        // Everything from the first '#' on is a comment.
        let code = match line.split_once('#') {
            Some((code, _comment)) => code,
            None => line,
        };
        let words: Vec<&str> = code.split_whitespace().collect();
        let [count, item] = words[..] else { continue };
        let Ok(count) = count.parse::<u32>() else { continue };
        if count == 0 {
            continue;
        }
        orders.push((count, item));
    }
    orders
}

pub fn total_quantity(text: &str, item: &str) -> u32 {
    let mut total = 0;
    for (count, name) in parse_orders(text) {
        if name == item {
            total += count;
        }
    }
    total
}
