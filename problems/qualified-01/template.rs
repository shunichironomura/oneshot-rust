pub struct Form {
    pub username: String,
    pub age: u8,
}

pub trait UsernameWidget {
    fn get(&self) -> String;
    fn kind() -> &'static str {
        "username"
    }
}

pub trait AgeWidget {
    fn get(&self) -> u8;
    fn kind() -> &'static str {
        "age"
    }
}

pub trait Labeled {
    fn label(&self) -> String;
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

impl Labeled for Form {
    fn label(&self) -> String {
        format!("user:{}", self.username)
    }
}

impl Form {
    /// The title shown above the form.
    pub fn label(&self) -> String {
        format!("form of {}", self.username)
    }
}

pub fn summary<W: UsernameWidget + AgeWidget>(w: &W) -> String {
    let name: String = w.get();
    let age: u8 = w.get();
    format!("{name} ({age})")
}

pub fn kinds<W: UsernameWidget + AgeWidget>() -> (&'static str, &'static str) {
    (UsernameWidget::kind(), AgeWidget::kind())
}

pub fn next_age(form: &Form) -> u8 {
    form.get().saturating_add(1)
}

pub fn tag(form: &Form) -> String {
    form.label()
}
