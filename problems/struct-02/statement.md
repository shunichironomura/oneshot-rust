A program keeps its settings in the `Config` struct below (it is already in the editor).
Implement `Default` for it and three functions that create configs.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub name: String,
    pub retries: u32,
    pub verbose: bool,
    pub tags: Vec<String>,
}

impl Default for Config { /* to implement */ }

pub fn renamed(base: &Config, name: &str) -> Config
pub fn quiet(base: Config) -> Config
pub fn with_tags(tags: &[&str]) -> Config
```

- `Config::default()` has the name `"app"`, `retries` set to `3`, `verbose` set to `false`, and no
  tags.
- `renamed` returns a copy of `base` whose name is `name`. All the other fields are the same as in
  `base`. `base` itself is only borrowed and stays as it was.
- `quiet` takes `base` by value and returns it with `verbose` set to `false` and `retries` set to
  `0`. The name and the tags are kept.
- `with_tags` returns the default config with `tags` as its tags, in the given order. Duplicates are
  kept.

### Examples

- `with_tags(&["a", "b"])` returns a config with the name `"app"`, `retries` set to `3`, `verbose`
  set to `false`, and the tags `["a", "b"]`.
- `renamed(&with_tags(&["x"]), "db")` returns a config with the name `"db"`, `retries` set to `3`,
  `verbose` set to `false`, and the tags `["x"]`.
