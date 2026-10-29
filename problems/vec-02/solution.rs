use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Job {
    pub id: u32,
    pub title: String,
    pub ttl: u32,
    pub done: bool,
}

pub fn dedup_titles(jobs: &mut Vec<Job>) -> usize {
    let before = jobs.len();
    let mut seen = HashSet::new();
    jobs.retain(|job| seen.insert(job.title.clone()));
    before - jobs.len()
}
