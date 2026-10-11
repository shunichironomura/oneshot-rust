#[derive(Debug, PartialEq)]
pub struct Job {
    pub id: u32,
    pub title: String,
    pub ttl: u32,
    pub done: bool,
}

pub fn take_done(jobs: &mut Vec<Job>) -> Vec<Job> {
    todo!()
}

pub fn tick(jobs: &mut Vec<Job>) -> Vec<u32> {
    todo!()
}

pub fn dedup_titles(jobs: &mut Vec<Job>) -> usize {
    todo!()
}

pub fn move_to_front(jobs: &mut Vec<Job>, id: u32) -> bool {
    todo!()
}
