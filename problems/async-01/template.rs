/// Resolves to a greeting for `name`.
pub fn greet(name: String) -> impl Future<Output = String> {
    async { format!("Hello, {name}!") }
}
