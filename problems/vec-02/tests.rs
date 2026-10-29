use crate::judge::{case, check, check_eq};
use crate::user::{Job, dedup_titles};

fn job(id: u32, title: &str, ttl: u32, done: bool) -> Job {
    Job { id, title: title.to_string(), ttl, done }
}

fn ids(jobs: &[Job]) -> Vec<u32> {
    jobs.iter().map(|j| j.id).collect()
}

pub fn run() {
    case("example-1", || {
        let mut jobs = vec![
            job(1, "a", 1, false),
            job(2, "b", 1, false),
            job(3, "a", 1, false),
            job(4, "c", 1, false),
            job(5, "b", 1, false),
        ];
        check_eq(dedup_titles(&mut jobs), 2)?;
        check_eq(ids(&jobs), vec![1, 2, 4])
    });
    case("case-1", || {
        // An empty list.
        let mut jobs: Vec<Job> = Vec::new();
        check_eq(dedup_titles(&mut jobs), 0)?;
        check(jobs.is_empty(), "the list should stay empty")
    });
    case("case-2", || {
        // No duplicates: titles are compared exactly.
        let mut jobs = vec![
            job(1, "Build", 1, false),
            job(2, "build", 1, false),
            job(3, "build ", 1, false),
            job(4, "", 1, false),
        ];
        check_eq(dedup_titles(&mut jobs), 0)?;
        check_eq(ids(&jobs), vec![1, 2, 3, 4])
    });
    case("case-3", || {
        // Only the title matters, and the first job with each title is kept.
        let mut jobs = vec![
            job(1, "x", 1, false),
            job(2, "y", 2, true),
            job(3, "y", 3, false),
            job(4, "x", 4, true),
            job(5, "", 5, false),
            job(6, "", 6, false),
            job(7, "z", 7, false),
            job(8, "x", 8, false),
        ];
        check_eq(dedup_titles(&mut jobs), 4)?;
        check_eq(
            &jobs,
            &vec![job(1, "x", 1, false), job(2, "y", 2, true), job(5, "", 5, false), job(7, "z", 7, false)],
        )?;
        let mut jobs = vec![job(9, "same", 1, false), job(8, "same", 1, false), job(7, "same", 1, false)];
        check_eq(dedup_titles(&mut jobs), 2)?;
        check_eq(ids(&jobs), vec![9])
    });
}
