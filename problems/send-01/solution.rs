use std::thread;

pub fn par_map<T, U, F>(items: Vec<T>, f: F) -> Vec<U>
where
    T: Send,
    U: Send,
    F: Fn(T) -> U + Sync,
{
    thread::scope(|s| {
        let f = &f;
        let handles: Vec<_> = items.into_iter().map(|x| s.spawn(move || f(x))).collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    })
}
