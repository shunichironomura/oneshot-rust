use std::fmt;

#[derive(Clone)]
pub struct Credentials {
    pub user: String,
    pub password: String,
    pub scopes: Vec<String>,
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("user", &self.user)
            // The `Debug` of `format_args!` writes the text as is. A `&str` would get quotes.
            .field("password", &format_args!("<redacted>"))
            .field("scopes", &self.scopes)
            .finish()
    }
}
