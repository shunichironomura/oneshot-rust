pub struct Player {
    pub name: String,
    pub score: u32,
}

/// The length of the nickname in bytes, or 0 if there is none.
pub fn name_len(nick: &Option<String>) -> usize {
    match nick {
        Some(ref s) => s.len(),
        None => 0,
    }
}

/// The name of the player with the highest score (the first one on a tie).
pub fn leader(players: &[Player]) -> Option<&str> {
    let mut best: Option<(&str, u32)> = None;
    for Player { ref name, score } in players {
        let better = match best {
            Some((_, top)) => *score > top,
            None => true,
        };
        if better {
            best = Some((name, *score));
        }
    }
    best.map(|(name, _)| name)
}

/// The sum of the scores. A name longer than 3 bytes earns 5 extra points.
pub fn bonus_total(scores: &[(String, u32)]) -> u32 {
    let mut total = 0;
    for (name, mut s) in scores {
        if name.len() > 3 {
            s += 5;
        }
        total += s;
    }
    total
}

/// The rounds (counted from 0) whose score is above `par`.
pub fn rounds_over(scores: &[u32], par: u32) -> Vec<usize> {
    scores
        .iter()
        .enumerate()
        .filter(|(_, &s)| s > par)
        .map(|(round, _)| round)
        .collect()
}

/// The initials, in order.
pub fn initials(entries: &[(char, u32)]) -> String {
    entries.iter().map(|&(c, _)| c).collect()
}
