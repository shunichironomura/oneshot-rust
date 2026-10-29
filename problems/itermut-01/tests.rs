use crate::judge::{case, check, check_eq};
use crate::user::runs_mut;

// The helpers take at most one run more than `v` has elements, so that an iterator that never
// ends (for example one that keeps yielding empty runs) fails the case instead of hanging.

fn front_lens<T: PartialEq>(v: &mut [T]) -> Vec<usize> {
    let limit = v.len() + 1;
    runs_mut(v).take(limit).map(|run| run.len()).collect()
}

fn back_lens<T: PartialEq>(v: &mut [T]) -> Vec<usize> {
    let limit = v.len() + 1;
    runs_mut(v).rev().take(limit).map(|run| run.len()).collect()
}

pub fn run() {
    case("example-1", || {
        let mut v = [1, 1, 2, 3, 3, 3];
        let runs: Vec<&mut [i32]> = runs_mut(&mut v).take(7).collect();
        check_eq(&runs, &[&[1, 1][..], &[2], &[3, 3, 3]])?;
        for run in runs {
            let len = run.len() as i32;
            run.fill(len);
        }
        check_eq(v, [2, 2, 1, 3, 3, 3])
    });
    case("example-2", || {
        let mut v = [1, 1, 2, 3, 3, 3];
        let rev: Vec<&mut [i32]> = runs_mut(&mut v).rev().take(7).collect();
        check_eq(rev, [&[3, 3, 3][..], &[2], &[1, 1]])
    });
    case("example-3", || {
        let mut v = [1, 1, 2, 3, 3, 3];
        let mut it = runs_mut(&mut v);
        check_eq(it.next().as_deref(), Some(&[1, 1][..]))?;
        check_eq(it.next_back().as_deref(), Some(&[3, 3, 3][..]))?;
        check_eq(it.next().as_deref(), Some(&[2][..]))?;
        check_eq(it.next().as_deref(), None)?;
        check_eq(it.next_back().as_deref(), None)
    });
    case("case-1", || {
        // An empty slice has no runs, whichever end is asked first.
        let mut v: [i32; 0] = [];
        let mut it = runs_mut(&mut v);
        check(it.next().is_none(), "next on an empty slice")?;
        check(it.next_back().is_none(), "next_back on an empty slice")?;
        let mut words: Vec<String> = Vec::new();
        check(runs_mut(&mut words).next_back().is_none(), "next_back first on an empty slice")?;
        // A single element, all equal, all distinct, and equal values that are not neighbors.
        let mut one = [42];
        let mut it = runs_mut(&mut one);
        check_eq(it.next().as_deref(), Some(&[42][..]))?;
        check(it.next_back().is_none() && it.next().is_none(), "[42] has one run")?;
        let mut same = [7, 7, 7, 7];
        let mut it = runs_mut(&mut same);
        check_eq(it.next_back().as_deref(), Some(&[7, 7, 7, 7][..]))?;
        check(it.next().is_none() && it.next_back().is_none(), "[7, 7, 7, 7] has one run")?;
        let mut distinct = [5, 4, 3, 2, 1];
        check_eq(front_lens(&mut distinct), vec![1; 5])?;
        let mut apart = [1, 2, 1, 1, 2];
        check_eq(front_lens(&mut apart), vec![1, 1, 2, 1])?;
        check_eq(back_lens(&mut apart), vec![1, 2, 1, 1])
    });
    case("case-2", || {
        // Elements that are not Copy, written through every run.
        let mut v: Vec<String> = ["a", "a", "b", "a"].map(String::from).to_vec();
        let runs: Vec<&mut [String]> = runs_mut(&mut v).take(5).collect();
        check_eq(runs.len(), 3)?;
        for (i, run) in runs.into_iter().enumerate() {
            for s in run.iter_mut() {
                s.push(char::from(b'0' + i as u8));
            }
        }
        check_eq(&v, &["a0", "a0", "b1", "a2"])?;
        let rev: Vec<&mut [String]> = runs_mut(&mut v).rev().take(5).collect();
        check_eq(rev, [&["a2"][..], &["b1"], &["a0", "a0"]])
    });
    case("case-3", || {
        // All runs are collected first, then read and written in any order.
        let mut v = [5, 5, 4, 4, 4, 3, 2, 2, 1, 1, 1, 1, 0];
        let mut runs: Vec<&mut [i32]> = runs_mut(&mut v).take(14).collect();
        check_eq(runs.iter().map(|run| run.len()).collect::<Vec<_>>(), vec![2, 3, 1, 2, 4, 1])?;
        let (front, back) = runs.split_at_mut(3);
        std::mem::swap(&mut front[0][1], &mut back[2][0]);
        back[0][1] = front[1].iter().sum();
        front[2][0] *= 10;
        check_eq(v, [5, 0, 4, 4, 4, 30, 2, 12, 1, 1, 1, 1, 5])?;
        // Only the given subslice is split, and runs outlive the iterator.
        let mut w = [1, 1, 1, 2, 2, 3, 3, 3];
        let mut it = runs_mut(&mut w[2..6]);
        let first = it.next();
        let second = it.next();
        drop(it);
        check_eq(first.as_deref(), Some(&[1][..]))?;
        if let Some(run) = second {
            run.fill(0);
        }
        check_eq(w, [1, 1, 1, 0, 0, 3, 3, 3])
    });
    case("case-4", || {
        // next and next_back strictly alternate until both ends meet.
        let mut v = [5, 5, 4, 4, 4, 3, 2, 2, 1, 1, 1, 1, 0];
        let mut it = runs_mut(&mut v);
        check_eq(it.next().as_deref(), Some(&[5, 5][..]))?;
        check_eq(it.next_back().as_deref(), Some(&[0][..]))?;
        check_eq(it.next().as_deref(), Some(&[4, 4, 4][..]))?;
        check_eq(it.next_back().as_deref(), Some(&[1, 1, 1, 1][..]))?;
        check_eq(it.next().as_deref(), Some(&[3][..]))?;
        check_eq(it.next_back().as_deref(), Some(&[2, 2][..]))?;
        check(it.next().is_none(), "next after the ends met")?;
        check(it.next_back().is_none(), "next_back after the ends met")?;
        check(it.next().is_none(), "next keeps returning None")?;
        // Runs taken from both ends are written at the same time.
        let mut w = [9, 8, 8, 9];
        let mut it = runs_mut(&mut w);
        let last = it.next_back().unwrap();
        let first = it.next().unwrap();
        let middle = it.next_back().unwrap();
        check(it.next().is_none() && it.next_back().is_none(), "[9, 8, 8, 9] has three runs")?;
        last[0] = 3;
        first[0] = 1;
        middle.fill(2);
        check_eq(w, [1, 2, 2, 3])
    });
    case("case-5", || {
        // NaN is not equal to itself, so every NaN is a run of its own.
        let nan = f64::NAN;
        let mut v = [nan, nan, 1.0, 1.0, nan];
        check_eq(front_lens(&mut v), vec![1, 1, 2, 1])?;
        check_eq(back_lens(&mut v), vec![1, 2, 1, 1])?;
        for run in runs_mut(&mut v).take(6) {
            let len = run.len() as f64;
            run.fill(len);
        }
        check_eq(v, [1.0, 1.0, 2.0, 2.0, 1.0])?;
        let mut tail = [0.5, 0.5, nan];
        check_eq(front_lens(&mut tail), vec![2, 1])?;
        check_eq(back_lens(&mut tail), vec![1, 2])
    });
}
