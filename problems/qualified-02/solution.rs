pub struct Form {
    pub username: String,
    pub age: u8,
}

pub trait UsernameWidget {
    fn get(&self) -> String;
}

pub trait AgeWidget {
    fn get(&self) -> u8;
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

pub fn summary<W: UsernameWidget + AgeWidget>(w: &W) -> String {
    let name = UsernameWidget::get(w);
    let age = AgeWidget::get(w);
    format!("{name} ({age})")
}
