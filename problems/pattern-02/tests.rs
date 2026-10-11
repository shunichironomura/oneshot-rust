use crate::judge::{case, check_eq};
use crate::user::{Player, bonus_total, initials, leader, name_len, rounds_over};

fn player(name: &str, score: u32) -> Player {
    Player { name: name.to_string(), score }
}

fn entry(name: &str, score: u32) -> (String, u32) {
    (name.to_string(), score)
}

pub fn run() {
    case("sample-1", || {
        check_eq(name_len(&Some("anna".to_string())), 4)?;
        let players = vec![player("anna", 3), player("bo", 9), player("cy", 4)];
        check_eq(leader(&players), Some("bo"))
    });
    case("sample-2", || check_eq(bonus_total(&[entry("anna", 1), entry("bo", 2)]), 8));
    case("sample-3", || {
        check_eq(rounds_over(&[3, 7, 5, 9], 5), vec![1, 3])?;
        check_eq(initials(&[('a', 1), ('b', 2)]), "ab")
    });
    case("hidden-1", || {
        // No nickname, an empty one, and a non-ASCII one (bytes, not chars).
        check_eq(name_len(&None), 0)?;
        check_eq(name_len(&Some(String::new())), 0)?;
        check_eq(name_len(&Some("héllo".to_string())), 6)
    });
    case("hidden-2", || {
        // On a tie the first player wins; a score of 0 can still lead; no players, no leader.
        let players = vec![player("ann", 5), player("ben", 7), player("cat", 7), player("dan", 2)];
        check_eq(leader(&players), Some("ben"))?;
        check_eq(leader(&[player("zero", 0)]), Some("zero"))?;
        let tied = vec![player("x", 4), player("y", 4)];
        check_eq(leader(&tied), Some("x"))?;
        check_eq(leader(&[]), None)
    });
    case("hidden-3", || {
        // Exactly 3 bytes earns nothing; the length is in bytes, not chars.
        check_eq(bonus_total(&[entry("abc", 10), entry("abcd", 10)]), 25)?;
        check_eq(bonus_total(&[entry("日本", 1), entry("éa", 1), entry("", 0)]), 7)?;
        check_eq(bonus_total(&[entry("long name", 0), entry("x", 4), entry("four", 6)]), 20)?;
        check_eq(bonus_total(&[]), 0)
    });
    case("hidden-4", || {
        // Equal to par is not above it.
        check_eq(rounds_over(&[5, 6, 5, 4, 6], 5), vec![1, 4])?;
        check_eq(rounds_over(&[0, 1, 0], 0), vec![1])?;
        check_eq(rounds_over(&[2, 2], 9), Vec::<usize>::new())?;
        check_eq(rounds_over(&[], 0), Vec::<usize>::new())
    });
    case("hidden-5", || {
        check_eq(initials(&[]), "")?;
        check_eq(initials(&[('日', 3), ('é', 0), ('z', 9)]), "日éz")
    });
}
