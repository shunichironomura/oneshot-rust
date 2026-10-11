use std::sync::mpsc::{Receiver, Sender};

pub fn send_all<T>(tx: &Sender<T>, items: Vec<T>) -> Vec<T> {
    todo!()
}

pub fn drain_ready<T>(rx: &Receiver<T>) -> (Vec<T>, bool) {
    todo!()
}

pub fn fan_in(groups: Vec<Vec<u32>>) -> Vec<u32> {
    todo!()
}
