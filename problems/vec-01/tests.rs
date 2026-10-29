use crate::judge::{case, check, check_eq};
use crate::user::{Job, dedup_titles, move_to_front, take_done, tick};

fn job(id: u32, title: &str, ttl: u32, done: bool) -> Job {
    Job { id, title: title.to_string(), ttl, done }
}

fn ids(jobs: &[Job]) -> Vec<u32> {
    jobs.iter().map(|j| j.id).collect()
}

fn titles(jobs: &[Job]) -> Vec<&str> {
    jobs.iter().map(|j| j.title.as_str()).collect()
}

pub fn run() {
    case("example-1", || {
        let mut jobs = vec![
            job(1, "a", 5, false),
            job(2, "b", 5, true),
            job(3, "c", 5, false),
            job(4, "d", 5, true),
        ];
        check_eq(take_done(&mut jobs), vec![job(2, "b", 5, true), job(4, "d", 5, true)])?;
        check_eq(jobs, vec![job(1, "a", 5, false), job(3, "c", 5, false)])
    });
    case("example-2", || {
        let mut jobs = vec![job(1, "a", 1, false), job(2, "b", 3, false), job(3, "c", 5, true)];
        check_eq(tick(&mut jobs), vec![1, 3])?;
        check_eq(jobs, vec![job(2, "b", 2, false)])
    });
    case("example-3", || {
        let mut jobs = vec![
            job(1, "a", 1, false),
            job(2, "b", 1, false),
            job(3, "a", 1, false),
            job(4, "c", 1, false),
            job(5, "b", 1, false),
        ];
        check_eq(dedup_titles(&mut jobs), 2)?;
        check_eq(ids(&jobs), vec![1, 2, 4])?;
        let mut jobs = vec![
            job(1, "a", 1, false),
            job(2, "b", 1, false),
            job(3, "c", 1, false),
            job(4, "d", 1, false),
        ];
        check_eq(move_to_front(&mut jobs, 3), true)?;
        check_eq(ids(&jobs), vec![3, 1, 2, 4])
    });
    case("case-1", || {
        // Every function works on an empty list.
        let mut jobs: Vec<Job> = Vec::new();
        check_eq(take_done(&mut jobs), Vec::<Job>::new())?;
        check_eq(tick(&mut jobs), Vec::<u32>::new())?;
        check_eq(dedup_titles(&mut jobs), 0)?;
        check_eq(move_to_front(&mut jobs, 0), false)?;
        check(jobs.is_empty(), "the list should stay empty")
    });
    case("case-2", || {
        // Nothing done: nothing changes.
        let mut jobs = vec![job(1, "a", 0, false), job(2, "b", 7, false)];
        check_eq(take_done(&mut jobs), Vec::<Job>::new())?;
        check_eq(&jobs, &vec![job(1, "a", 0, false), job(2, "b", 7, false)])?;
        // Both parts keep their order.
        let mut jobs = vec![
            job(5, "e", 1, true),
            job(6, "f", 2, false),
            job(7, "g", 3, true),
            job(8, "h", 4, true),
            job(9, "i", 5, false),
            job(10, "j", 6, true),
        ];
        check_eq(
            take_done(&mut jobs),
            vec![job(5, "e", 1, true), job(7, "g", 3, true), job(8, "h", 4, true), job(10, "j", 6, true)],
        )?;
        check_eq(&jobs, &vec![job(6, "f", 2, false), job(9, "i", 5, false)])?;
        // Everything done.
        let mut jobs = vec![job(1, "a", 1, true), job(2, "b", 2, true)];
        check_eq(take_done(&mut jobs), vec![job(1, "a", 1, true), job(2, "b", 2, true)])?;
        check(jobs.is_empty(), "every job should be taken")
    });
    case("case-3", || {
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
    case("case-4", || {
        // Ticking until everything is gone.
        let mut jobs = vec![job(1, "a", 2, false), job(2, "b", 3, false), job(3, "c", 2, false)];
        check_eq(tick(&mut jobs), Vec::<u32>::new())?;
        check_eq(jobs.iter().map(|j| j.ttl).collect::<Vec<_>>(), vec![1, 2, 1])?;
        check_eq(tick(&mut jobs), vec![1, 3])?;
        check_eq(&jobs, &vec![job(2, "b", 1, false)])?;
        check_eq(tick(&mut jobs), vec![2])?;
        check(jobs.is_empty(), "every job should be removed")
    });
    case("case-5", || {
        // No duplicates: titles are compared exactly.
        let mut jobs = vec![
            job(1, "Build", 1, false),
            job(2, "build", 1, false),
            job(3, "build ", 1, false),
            job(4, "", 1, false),
        ];
        check_eq(dedup_titles(&mut jobs), 0)?;
        check_eq(ids(&jobs), vec![1, 2, 3, 4])?;
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
    case("case-6", || {
        let mut jobs = vec![
            job(1, "a", 1, false),
            job(2, "b", 2, true),
            job(3, "c", 3, false),
            job(4, "d", 4, false),
            job(5, "e", 5, true),
        ];
        // Already at the front.
        check_eq(move_to_front(&mut jobs, 1), true)?;
        check_eq(ids(&jobs), vec![1, 2, 3, 4, 5])?;
        // Absent.
        check_eq(move_to_front(&mut jobs, 9), false)?;
        check_eq(ids(&jobs), vec![1, 2, 3, 4, 5])?;
        // The others keep their order.
        check_eq(move_to_front(&mut jobs, 5), true)?;
        check_eq(ids(&jobs), vec![5, 1, 2, 3, 4])?;
        check_eq(move_to_front(&mut jobs, 3), true)?;
        check_eq(
            jobs,
            vec![
                job(3, "c", 3, false),
                job(5, "e", 5, true),
                job(1, "a", 1, false),
                job(2, "b", 2, true),
                job(4, "d", 4, false),
            ],
        )
    });
    case("case-7", || {
        // Only the first job with the id moves.
        let mut jobs = vec![
            job(1, "a", 1, false),
            job(2, "b", 1, false),
            job(3, "c", 1, false),
            job(2, "d", 1, false),
            job(4, "e", 1, false),
        ];
        check_eq(move_to_front(&mut jobs, 2), true)?;
        check_eq(titles(&jobs), vec!["b", "a", "c", "d", "e"])?;
        check_eq(move_to_front(&mut jobs, 2), true)?;
        check_eq(titles(&jobs), vec!["b", "a", "c", "d", "e"])?;
        let mut one = vec![job(7, "solo", 1, false)];
        check_eq(move_to_front(&mut one, 8), false)?;
        check_eq(move_to_front(&mut one, 7), true)?;
        check_eq(one, vec![job(7, "solo", 1, false)])
    });
}
