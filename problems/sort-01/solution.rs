use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct Run {
    pub name: String,
    pub score: u32,
    pub time: u32,
}

/// Better runs come first: higher score, then shorter time.
fn by_quality(a: &Run, b: &Run) -> Ordering {
    b.score.cmp(&a.score).then_with(|| a.time.cmp(&b.time))
}

pub fn leaderboard(mut runs: Vec<Run>) -> Vec<Run> {
    // Group runs by player with each player's best run first, then keep only that one.
    runs.sort_by(|a, b| a.name.cmp(&b.name).then_with(|| by_quality(a, b)));
    runs.dedup_by(|later, first| later.name == first.name);

    runs.sort_by(|a, b| by_quality(a, b).then_with(|| a.name.cmp(&b.name)));
    runs
}
