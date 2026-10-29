#[derive(Debug, PartialEq)]
pub struct Job {
    pub id: u32,
    pub title: String,
    pub ttl: u32,
    pub done: bool,
}

pub fn tick(jobs: &mut Vec<Job>) -> Vec<u32> {
    todo!()
}
