use std::sync::{Mutex, Condvar};

pub struct Semaphore {
    count: Mutex<usize>,
    is_available: Condvar,
    max: usize,
}

impl Semaphore {
    pub fn new(max: usize) -> Self {
        Self {
            count: Mutex::new(max),
            is_available: Condvar::new(),
            max,
        }
    }

    pub fn down(&self) {
        let mut counter = self.count.lock().unwrap();

        while *counter == 0 {
            counter = self.is_available.wait(counter).unwrap();
        }

        *counter -= 1;
    }

    pub fn up(&self) {
        let mut counter = self.count.lock().unwrap();

        if *counter < self.max {
            *counter += 1;
            self.is_available.notify_one();
        }
    }
}