use std::thread::{self, JoinHandle};

pub fn par_map<T, U, F>(items: Vec<T>, f: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    thread::scope(|s| {
        let f = &f;
        let handles: Vec<_> = items.into_iter().map(|x| s.spawn(move || f(x))).collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    })
}

pub fn spawn_job<T, U, F>(input: T, f: F) -> JoinHandle<U>
where
    F: FnMut(T) -> U,
{
    thread::spawn(move || f(input))
}
