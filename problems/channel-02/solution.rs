use std::sync::mpsc::Sender;

pub fn send_all<T>(tx: &Sender<T>, items: Vec<T>) -> Vec<T> {
    let mut unsent = Vec::new();
    for item in items {
        // `send` takes the item by value; on failure, the error hands it back.
        if let Err(err) = tx.send(item) {
            unsent.push(err.0);
        }
    }
    unsent
}
