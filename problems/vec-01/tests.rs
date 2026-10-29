use crate::judge::{case, check, check_eq};
use crate::user::{Job, tick};

fn job(id: u32, title: &str, ttl: u32, done: bool) -> Job {
    Job { id, title: title.to_string(), ttl, done }
}

pub fn run() {
    case("example-1", || {
        let mut jobs = vec![job(1, "a", 1, false), job(2, "b", 3, false), job(3, "c", 5, true)];
        check_eq(tick(&mut jobs), vec![1, 3])?;
        check_eq(jobs, vec![job(2, "b", 2, false)])
    });
    case("case-1", || {
        // An empty list.
        let mut jobs: Vec<Job> = Vec::new();
        check_eq(tick(&mut jobs), Vec::<u32>::new())?;
        check(jobs.is_empty(), "the list should stay empty")
    });
    case("case-2", || {
        // A ttl of 0 stays 0, and the job is removed. Ids come back in their original order.
        let mut jobs = vec![
            job(1, "a", 0, false),
            job(2, "b", 2, false),
            job(3, "c", 9, true),
            job(4, "d", 1, false),
            job(5, "e", u32::MAX, false),
            job(6, "f", 0, true),
        ];
        check_eq(tick(&mut jobs), vec![1, 3, 4, 6])?;
        check_eq(jobs, vec![job(2, "b", 1, false), job(5, "e", u32::MAX - 1, false)])
    });
    case("case-3", || {
        // Ticking until everything is gone.
        let mut jobs = vec![job(1, "a", 2, false), job(2, "b", 3, false), job(3, "c", 2, false)];
        check_eq(tick(&mut jobs), Vec::<u32>::new())?;
        check_eq(jobs.iter().map(|j| j.ttl).collect::<Vec<_>>(), vec![1, 2, 1])?;
        check_eq(tick(&mut jobs), vec![1, 3])?;
        check_eq(&jobs, &vec![job(2, "b", 1, false)])?;
        check_eq(tick(&mut jobs), vec![2])?;
        check(jobs.is_empty(), "every job should be removed")
    });
}
