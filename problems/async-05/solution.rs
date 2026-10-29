use std::pin::Pin;

/// Awaits the futures one after another and adds up their outputs.
pub async fn sum_all(futs: Vec<Pin<Box<dyn Future<Output = u32>>>>) -> u32 {
    let mut total = 0;
    // A `for` loop has no closure, so `.await` is directly inside the async fn.
    for f in futs {
        total += f.await;
    }
    total
}
