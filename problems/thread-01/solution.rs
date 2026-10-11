use std::thread;

pub fn parallel_sum(values: &[u64], workers: usize) -> u64 {
    let chunk_size = values.len().div_ceil(workers).max(1);
    thread::scope(|s| {
        let mut handles = Vec::new();
        for chunk in values.chunks(chunk_size) {
            handles.push(s.spawn(move || chunk.iter().sum::<u64>()));
        }
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    })
}
