pub fn tally(events: &[&str], limit: usize) -> (usize, Vec<String>) {
    let mut errors = 0;
    let mut log = Vec::new();
    let mut record = |event: &str| {
        if event.starts_with("ERR") {
            errors += 1;
        }
        log.push(event.to_string());
    };
    for event in events {
        record(event);
        if errors == limit {
            log.push("abort".to_string());
            break;
        }
    }
    (errors, log)
}
