use std::collections::BTreeMap;

pub struct Timeline {
    /// Time -> description. A `BTreeMap` keeps its keys sorted.
    events: BTreeMap<u32, String>,
}

impl Timeline {
    pub fn new() -> Self {
        Timeline { events: BTreeMap::new() }
    }

    pub fn add(&mut self, t: u32, what: &str) -> Option<String> {
        self.events.insert(t, what.to_string())
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn between(&self, from: u32, to: u32) -> Vec<(u32, &str)> {
        // `range` panics when the start is greater than the end.
        if from > to {
            return Vec::new();
        }
        self.events
            .range(from..=to)
            .map(|(&time, what)| (time, what.as_str()))
            .collect()
    }

    pub fn last_before(&self, t: u32) -> Option<(u32, &str)> {
        self.events
            .range(..t)
            .next_back()
            .map(|(&time, what)| (time, what.as_str()))
    }

    pub fn next_at_or_after(&self, t: u32) -> Option<(u32, &str)> {
        self.events
            .range(t..)
            .next()
            .map(|(&time, what)| (time, what.as_str()))
    }

    pub fn split_from(&mut self, t: u32) -> Timeline {
        Timeline { events: self.events.split_off(&t) }
    }

    pub fn prune_before(&mut self, t: u32) -> usize {
        let kept = self.events.split_off(&t);
        let removed = std::mem::replace(&mut self.events, kept);
        removed.len()
    }
}
