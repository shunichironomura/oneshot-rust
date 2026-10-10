pub fn greet_all(names: Vec<String>) -> (Vec<String>, usize) {
    let mut greetings = Vec::new();
    for name in names {
        greetings.push(format!("Hello, {name}!"));
    }
    (greetings, names.len())
}
