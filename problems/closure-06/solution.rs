pub fn make_greeter(greeting: String) -> impl Fn(&str) -> String {
    // An owned copy of the trimmed text: a `&str` would still borrow the parameter.
    let greeting = greeting.trim().to_string();
    move |name| format!("{greeting}, {name}!")
}
