#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub name: String,
    pub retries: u32,
    pub verbose: bool,
    pub tags: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: String::from("app"),
            retries: 3,
            verbose: false,
            tags: Vec::new(),
        }
    }
}

pub fn renamed(base: &Config, name: &str) -> Config {
    Config {
        name: name.to_string(),
        ..base.clone()
    }
}

pub fn quiet(base: Config) -> Config {
    Config {
        retries: 0,
        verbose: false,
        ..base
    }
}

pub fn with_tags(tags: &[&str]) -> Config {
    let tags = tags.iter().map(|tag| tag.to_string()).collect();
    Config {
        tags,
        ..Default::default()
    }
}

pub fn describe(cfg: &Config) -> String {
    let Config { name, retries, verbose, tags } = cfg;
    format!("{name}: retries={retries}, verbose={verbose}, tags={}", tags.join(","))
}
