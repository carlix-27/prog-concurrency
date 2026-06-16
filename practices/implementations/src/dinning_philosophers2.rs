use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct Table {
    forks: Mutex<Vec<bool>>, // true = libre, false = ocupado
    can_eat: Condvar,
}

struct Philosopher {
    table: Arc<Table>,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn think(&self) {
        println!("Philosopher is thinking");
        thread::sleep(Duration::from_millis(50));
    }

    fn eat(&self) {
        {
            let mut forks = self.table.forks.lock().unwrap();

            while !forks[self.left] || !forks[self.right] {
                forks = self.table.can_eat.wait(forks).unwrap();
            }

            forks[self.left] = false;
            forks[self.right] = false;
        }

        println!("Philosopher eating");
        thread::sleep(Duration::from_millis(100));

        {
            let mut forks = self.table.forks.lock().unwrap();

            forks[self.left] = true;
            forks[self.right] = true;

            self.table.can_eat.notify_all();
        }
    }
}