pub fn join_all<F: Future>(futs: Vec<F>) -> impl Future<Output = Vec<F::Output>> {
    std::future::poll_fn(|_| todo!())
}
