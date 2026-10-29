pub struct Form;

pub trait UsernameWidget {
    fn kind() -> &'static str {
        "username"
    }
}

pub trait AgeWidget {
    fn kind() -> &'static str {
        "age"
    }
}

impl UsernameWidget for Form {}

impl AgeWidget for Form {}

pub fn kinds<W: UsernameWidget + AgeWidget>() -> (&'static str, &'static str) {
    (<W as UsernameWidget>::kind(), <W as AgeWidget>::kind())
}
