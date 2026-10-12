use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    /// `None` once the pool has been stopped.
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "a thread pool needs at least one worker");
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = (0..size)
            .map(|_| {
                let receiver = Arc::clone(&receiver);
                thread::spawn(move || run_worker(&receiver))
            })
            .collect();
        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let sender = self.sender.as_ref().expect("the pool is running");
        sender.send(Box::new(f)).expect("the workers are running");
    }

    pub fn shutdown(mut self) {
        self.stop();
        // `self` is dropped here; `drop` finds nothing left to stop.
    }

    /// Closes the queue and waits for every worker to end.
    fn stop(&mut self) {
        // Dropping the only `Sender` closes the channel: once the queue is empty, `recv` fails
        // and every worker leaves its loop.
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            worker.join().unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.stop();
    }
}

fn run_worker(receiver: &Mutex<Receiver<Job>>) {
    loop {
        // The guard is a temporary of this `let` statement, so the lock is released before the
        // job runs and the other workers can take jobs meanwhile.
        let message = receiver.lock().unwrap().recv();
        match message {
            Ok(job) => job(),
            Err(_) => return,
        }
    }
}
