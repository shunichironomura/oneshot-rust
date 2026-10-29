use std::pin::Pin;

/// Awaits the futures one after another and adds up their outputs.
pub async fn sum_all(futs: Vec<Pin<Box<dyn Future<Output = u32>>>>) -> u32 {
    futs.into_iter().map(|f| f.await).sum()
}
