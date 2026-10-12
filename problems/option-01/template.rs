#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub username: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
}

impl Profile {
    pub fn display_name(&self) -> &str {
        match self.nickname {
            Some(nick) => &nick,
            None => &self.username,
        }
    }

    pub fn email_domain(&self) -> Option<&str> {
        let email = self.email?;
        email.split_once('@').map(|(_, domain)| domain)
    }

    pub fn take_nickname(&mut self) -> Option<String> {
        let old = self.nickname;
        self.nickname = None;
        old
    }
}
