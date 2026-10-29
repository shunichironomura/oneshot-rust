use std::fmt;

#[derive(Clone)]
pub struct Credentials {
    pub user: String,
    pub password: String,
    pub scopes: Vec<String>,
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
