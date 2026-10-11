use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Job {
    pub id: u32,
    pub title: String,
    pub ttl: u32,
    pub done: bool,
}

pub fn take_done(jobs: &mut Vec<Job>) -> Vec<Job> {
    jobs.extract_if(.., |job| job.done).collect()
}

pub fn tick(jobs: &mut Vec<Job>) -> Vec<u32> {
    jobs.extract_if(.., |job| {
        job.ttl = job.ttl.saturating_sub(1);
        job.done || job.ttl == 0
    })
    .map(|job| job.id)
    .collect()
}

pub fn dedup_titles(jobs: &mut Vec<Job>) -> usize {
    let before = jobs.len();
    let mut seen = HashSet::new();
    jobs.retain(|job| seen.insert(job.title.clone()));
    before - jobs.len()
}

pub fn move_to_front(jobs: &mut Vec<Job>, id: u32) -> bool {
    match jobs.iter().position(|job| job.id == id) {
        Some(i) => {
            jobs[..=i].rotate_right(1);
            true
        }
        None => false,
    }
}
