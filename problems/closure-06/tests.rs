use crate::judge::{case, check_eq};
use crate::user::make_greeter;

pub fn run() {
    case("example-1", || {
        let hi = make_greeter(String::from("  Hi "));
        check_eq(hi("Ann"), "Hi, Ann!")
    });
    case("case-1", || {
        // Greeters outlive the Strings they were made from and can be called many times.
        // `trim` removes Unicode whitespace (U+3000) at both ends but keeps inner whitespace.
        let mut greeters = Vec::new();
        for greeting in ["\tHola\n", "\u{3000}こんにちは ", "   ", " Good  day "] {
            greeters.push(make_greeter(greeting.to_string()));
        }
        let name = String::from("Ferris");
        let got: Vec<String> = greeters.iter().rev().map(|g| g(&name)).collect();
        check_eq(
            got,
            ["Good  day, Ferris!", ", Ferris!", "こんにちは, Ferris!", "Hola, Ferris!"],
        )?;
        drop(name);
        check_eq(greeters[0](" Bob "), "Hola,  Bob !")
    });
}
