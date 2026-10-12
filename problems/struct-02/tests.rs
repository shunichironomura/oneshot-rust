use crate::judge::{case, check_eq};
use crate::user::{Config, quiet, renamed, with_tags};

fn config(name: &str, retries: u32, verbose: bool, tags: &[&str]) -> Config {
    Config {
        name: name.to_string(),
        retries,
        verbose,
        tags: tags.iter().map(|t| t.to_string()).collect(),
    }
}

pub fn run() {
    case("sample-1", || {
        check_eq(Config::default(), config("app", 3, false, &[]))?;
        check_eq(with_tags(&["a", "b"]), config("app", 3, false, &["a", "b"]))
    });
    case("sample-2", || check_eq(renamed(&with_tags(&["x"]), "db"), config("db", 3, false, &["x"])));
    case("hidden-1", || {
        // with_tags with no tags is the default config.
        check_eq(with_tags(&[]), Config::default())
    });
    case("hidden-2", || {
        // renamed keeps every other field, and the base stays as it was.
        let base = config("svc", 7, true, &["blue", "green"]);
        let copy = renamed(&base, "svc-2");
        check_eq(&copy, &config("svc-2", 7, true, &["blue", "green"]))?;
        check_eq(&base, &config("svc", 7, true, &["blue", "green"]))?;
        check_eq(renamed(&copy, "svc"), base)
    });
    case("hidden-3", || {
        // quiet turns verbose off and retries to 0, and keeps the name and the tags.
        check_eq(quiet(config("svc", 5, true, &["x"])), config("svc", 0, false, &["x"]))?;
        check_eq(quiet(Config::default()), config("app", 0, false, &[]))
    });
    case("hidden-4", || {
        // with_tags keeps the order and the duplicates.
        check_eq(with_tags(&["b", "a", "b"]), config("app", 3, false, &["b", "a", "b"]))
    });
    case("hidden-5", || {
        // quiet of a config built by with_tags.
        check_eq(quiet(with_tags(&["p", "q", "r"])), config("app", 0, false, &["p", "q", "r"]))
    });
    case("hidden-6", || {
        // renamed of a quiet config.
        let q = quiet(config("web", 9, true, &["eu"]));
        let r = renamed(&q, "api");
        check_eq(r, config("api", 0, false, &["eu"]))?;
        check_eq(q, config("web", 0, false, &["eu"]))
    });
}
