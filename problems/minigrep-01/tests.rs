use crate::judge::{case, check_eq};
use crate::user::{Config, search, search_case_insensitive};

fn build(argv: &[&str]) -> Result<Config, &'static str> {
    let args: Vec<String> = argv.iter().map(|s| s.to_string()).collect();
    Config::build(args.into_iter())
}

fn config(query: &str, file_path: &str, ignore_case: bool) -> Config {
    Config { query: query.to_string(), file_path: file_path.to_string(), ignore_case }
}

const POEM: &str = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

pub fn run() {
    case("sample-1", || {
        check_eq(build(&["minigrep", "to", "poem.txt"]), Ok(config("to", "poem.txt", false)))
    });
    case("sample-2", || {
        check_eq(search("duct", POEM), vec!["safe, fast, productive."])?;
        check_eq(search_case_insensitive("rUsT", POEM), vec!["Rust:", "Trust me."])
    });
    case("hidden-1", || {
        // Missing arguments, including a completely empty iterator.
        check_eq(Config::build(std::iter::empty()), Err("Didn't get a query string"))?;
        check_eq(build(&["minigrep"]), Err("Didn't get a query string"))?;
        check_eq(build(&["minigrep", "needle"]), Err("Didn't get a file path"))
    });
    case("hidden-2", || {
        // The optional fourth argument, passed through another kind of iterator.
        let argv = ["minigrep", "body", "poem.txt", "--ignore-case"];
        check_eq(
            Config::build(argv.iter().map(|s| s.to_string())),
            Ok(config("body", "poem.txt", true)),
        )?;
        // Only positions matter: empty strings are arguments, and "--ignore-case" in another
        // position is an ordinary value.
        check_eq(build(&["", "", ""]), Ok(config("", "", false)))?;
        check_eq(
            build(&["--ignore-case", "--ignore-case", "x"]),
            Ok(config("--ignore-case", "x", false)),
        )?;
        check_eq(build(&["minigrep", "q", "--ignore-case"]), Ok(config("q", "--ignore-case", false)))
    });
    case("hidden-3", || {
        // Anything else in fourth place, and any fifth argument, is rejected.
        check_eq(build(&["minigrep", "a", "b", "-i"]), Err("Unexpected argument"))?;
        check_eq(build(&["minigrep", "a", "b", ""]), Err("Unexpected argument"))?;
        check_eq(build(&["minigrep", "a", "b", "--ignore-case", "c"]), Err("Unexpected argument"))?;
        check_eq(build(&["minigrep", "a", "b", "c", "--ignore-case"]), Err("Unexpected argument"))
    });
    case("hidden-4", || {
        // Lines are split like str::lines. Matching is case-sensitive, and each matching line
        // appears once, in order. The empty query matches every line.
        check_eq(
            search("rust", "Rust\ntrust\r\nbar\ntrust rust\ntrust"),
            vec!["trust", "trust rust", "trust"],
        )?;
        check_eq(search("", "one\r\ntwo\n\nthree\n"), vec!["one", "two", "", "three"])?;
        check_eq(search("", ""), Vec::<&str>::new())
    });
    case("hidden-5", || {
        // Unicode lowercasing, and results that borrow only from contents.
        let contents = String::from("ÉCOLE\nécole\r\nECOLE\n");
        let query = String::from("ÉCOLE");
        let exact = search(&query, &contents);
        let any_case = search_case_insensitive(&query, &contents);
        drop(query);
        check_eq(exact, vec!["ÉCOLE"])?;
        check_eq(any_case, vec!["ÉCOLE", "école"])?;
        check_eq(search_case_insensitive("", "A\r\nb\n"), vec!["A", "b"])
    });
}
