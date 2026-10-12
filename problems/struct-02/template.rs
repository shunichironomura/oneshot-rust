#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub name: String,
    pub retries: u32,
    pub verbose: bool,
    pub tags: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        todo!()
    }
}

pub fn renamed(base: &Config, name: &str) -> Config {
    todo!()
}

pub fn quiet(base: Config) -> Config {
    todo!()
}

pub fn with_tags(tags: &[&str]) -> Config {
    todo!()
}
