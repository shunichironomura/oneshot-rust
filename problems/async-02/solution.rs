/// Calls `f` until it returns `Ok` or has been called `times` times,
/// and returns the last result.
pub async fn retry<T, E, F>(times: usize, mut f: F) -> Result<T, E>
where
    F: AsyncFnMut() -> Result<T, E>,
{
    let mut last = f().await;
    for _ in 1..times {
        if last.is_ok() {
            break;
        }
        last = f().await;
    }
    last
}

/// Calls `fetch` with `{mirror}/{path}` for each mirror in order,
/// until a call returns `Ok`, and returns the last result.
pub async fn fetch_any<T, E, F>(mirrors: &[&str], path: &str, mut fetch: F) -> Result<T, E>
where
    F: AsyncFnMut(&str) -> Result<T, E>,
{
    let mut urls = mirrors.iter().map(|mirror| format!("{mirror}/{path}"));
    retry(mirrors.len(), async || {
        let url = urls.next().unwrap();
        fetch(&url).await
    })
    .await
}
