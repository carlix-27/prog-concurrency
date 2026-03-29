mod worker;

use std::{
    sync::{Arc, Mutex, mpsc},
};
use std::sync::mpsc::Receiver;
use crate::worker::Worker;

pub trait FnOnce{}


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;

/// Create a new ThreadPool.
/// The size is the number of threads in the pool.
/// # Panics
/// The `new` function will panic if the size is zero. (todo: ThreadPoolError -> Result. Ok | Error)


impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver_job: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let thread_receiver: Arc<Mutex<Receiver<Job>>> = Arc::clone(&receiver_job);

            workers.push(Worker::new(id, thread_receiver));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

pub trait Drop {
    // Required method
    fn drop(&mut self);
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
