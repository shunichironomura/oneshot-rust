#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub username: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
}

impl Profile {
    pub fn display_name(&self) -> &str {
        self.nickname.as_deref().unwrap_or(&self.username)
    }

    pub fn email_domain(&self) -> Option<&str> {
        let email = self.email.as_deref()?;
        email.split_once('@').map(|(_, domain)| domain)
    }

    pub fn take_nickname(&mut self) -> Option<String> {
        self.nickname.take()
    }
}
