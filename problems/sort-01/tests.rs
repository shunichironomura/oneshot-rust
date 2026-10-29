use crate::judge::{case, check_eq};
use crate::user::{Run, leaderboard};

fn runs(xs: &[(&str, u32, u32)]) -> Vec<Run> {
    xs.iter()
        .map(|&(name, score, time)| Run { name: name.to_string(), score, time })
        .collect()
}

pub fn run() {
    case("example-1", || {
        check_eq(
            leaderboard(runs(&[("eve", 80, 30), ("bob", 90, 50), ("eve", 95, 40), ("amy", 90, 45)])),
            runs(&[("eve", 95, 40), ("amy", 90, 45), ("bob", 90, 50)]),
        )
    });
    case("example-2", || {
        check_eq(
            leaderboard(runs(&[("cat", 70, 20), ("ann", 70, 20), ("bob", 70, 20)])),
            runs(&[("ann", 70, 20), ("bob", 70, 20), ("cat", 70, 20)]),
        )
    });
    case("case-1", || check_eq(leaderboard(Vec::new()), Vec::new()));
    case("case-2", || {
        // Same best score for one player: the faster run is kept.
        check_eq(
            leaderboard(runs(&[("zed", 50, 90), ("zed", 50, 30), ("zed", 10, 5)])),
            runs(&[("zed", 50, 30)]),
        )
    });
    case("case-3", || {
        check_eq(
            leaderboard(runs(&[
                ("b", 10, 9),
                ("a", 10, 9),
                ("c", 0, 1),
                ("b", 10, 3),
                ("d", 10, 3),
                ("a", 5, 1),
            ])),
            runs(&[("b", 10, 3), ("d", 10, 3), ("a", 10, 9), ("c", 0, 1)]),
        )
    });
    case("case-4", || {
        // Names are compared exactly (case-sensitive), so these are three players.
        check_eq(
            leaderboard(runs(&[("Bo", 1, 1), ("bo", 1, 1), ("BO", 2, 1)])),
            runs(&[("BO", 2, 1), ("Bo", 1, 1), ("bo", 1, 1)]),
        )
    });
}
