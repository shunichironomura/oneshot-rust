use std::collections::BTreeMap;

pub struct Timeline {
    /// Time -> description. A `BTreeMap` keeps its keys sorted.
    events: BTreeMap<u32, String>,
}

impl Timeline {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add(&mut self, t: u32, what: &str) -> Option<String> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn between(&self, from: u32, to: u32) -> Vec<(u32, &str)> {
        todo!()
    }

    pub fn last_before(&self, t: u32) -> Option<(u32, &str)> {
        todo!()
    }

    pub fn next_at_or_after(&self, t: u32) -> Option<(u32, &str)> {
        todo!()
    }

    pub fn split_from(&mut self, t: u32) -> Timeline {
        todo!()
    }

    pub fn prune_before(&mut self, t: u32) -> usize {
        todo!()
    }
}
