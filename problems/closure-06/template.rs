pub fn make_greeter(greeting: String) -> impl Fn(&str) -> String {
    let greeting = greeting.trim();
    |name| format!("{greeting}, {name}!")
}
