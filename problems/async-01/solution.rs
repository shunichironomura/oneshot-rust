/// Resolves to a greeting for `name`.
pub fn greet(name: String) -> impl Future<Output = String> {
    // `move` puts `name` inside the future, so the future does not borrow a local of `greet`.
    async move { format!("Hello, {name}!") }
}
