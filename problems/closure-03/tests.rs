use crate::judge::{case, check_eq};
use crate::user::{make_greeter, tally};

fn strings(xs: &[&str]) -> Vec<String> {
    xs.iter().map(|&s| String::from(s)).collect()
}

pub fn run() {
    case("sample-1", || {
        check_eq(
            tally(&["ok", "ERR a", "ERR b", "ok"], 2),
            (2, strings(&["ok", "ERR a", "ERR b", "abort"])),
        )?;
        check_eq(tally(&["ok"], 1), (0, strings(&["ok"])))
    });
    case("sample-2", || {
        let hi = make_greeter(String::from("  Hi "));
        check_eq(hi("Ann"), "Hi, Ann!")
    });
    case("hidden-1", || {
        // With limit 1, the first error aborts at once.
        check_eq(tally(&["ERR"], 1), (1, strings(&["ERR", "abort"])))?;
        check_eq(tally(&["ERR x", "ok", "ERR y"], 1), (1, strings(&["ERR x", "abort"])))
    });
    case("hidden-2", || {
        // The limit is reached by the last event: "abort" is still appended.
        check_eq(
            tally(&["ok", "ERR 1", "ok", "ERR 2"], 2),
            (2, strings(&["ok", "ERR 1", "ok", "ERR 2", "abort"])),
        )
    });
    case("hidden-3", || {
        // The limit is never reached, and an empty list.
        check_eq(tally(&["ERR a", "ok", "ERR b"], 3), (2, strings(&["ERR a", "ok", "ERR b"])))?;
        check_eq(tally(&[], 1), (0, Vec::new()))
    });
    case("hidden-4", || {
        // Only a case-sensitive "ERR" at the very start counts.
        check_eq(
            tally(&["err x", "Err", " ERR", "xERR", "ERROR", "ok", "ERR"], 2),
            (2, strings(&["err x", "Err", " ERR", "xERR", "ERROR", "ok", "ERR", "abort"])),
        )
    });
    case("hidden-5", || {
        // Events after the abort are neither logged nor counted.
        check_eq(
            tally(&["ERR a", "ERR b", "ERR c", "ok", "ERR d"], 2),
            (2, strings(&["ERR a", "ERR b", "abort"])),
        )
    });
    case("hidden-6", || {
        // Greeters outlive the Strings they were made from and can be called many times.
        let mut greeters = Vec::new();
        for greeting in ["\tHola\n", " こんにちは ", "   "] {
            greeters.push(make_greeter(greeting.to_string()));
        }
        let name = String::from("Ferris");
        let got: Vec<String> = greeters.iter().rev().map(|g| g(&name)).collect();
        check_eq(got, strings(&[", Ferris!", "こんにちは, Ferris!", "Hola, Ferris!"]))?;
        drop(name);
        check_eq(greeters[0](" Bob "), "Hola,  Bob !")?;
        check_eq(greeters[1](""), "こんにちは, !")
    });
}
