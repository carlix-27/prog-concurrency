use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Copy, Clone)]
struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up forks...
        let _left_fork_guard = self.left_fork.lock().unwrap();
        let _right_fork_guard = self.right_fork.lock().unwrap();
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    // Create forks
    let forks = std::iter::repeat_with(||Arc::new(Mutex::new(Fork)))
        .take(PHILOSOPHERS.len())
        .collect::<Vec<_>>();
    // Create philosophers
    let (tx, rx) = mpsc::channel();
    let philosophers = PHILOSOPHERS.iter().enumerate().map(|(i, &name)| {
        if i == PHILOSOPHERS.len() - 1 {
            Philosopher {
                name: name.into(),
                left_fork: Arc::clone(&forks[(i + 1) % forks.len()]),
                right_fork: Arc::clone(&forks[i]),
                thoughts: tx.clone(),
            }
        } else {
            Philosopher {
                name: name.into(),
                left_fork: Arc::clone(&forks[i]),
                right_fork: Arc::clone(&forks[(i + 1) % forks.len()]),
                thoughts: tx.clone(),
            }
        }
    });
    // Make each of them think and eat 100 times
    thread::scope(|s| {
        for philosopher in philosophers {
            s.spawn(move || {
                for _ in 0..100 {
                    philosopher.think();
                    philosopher.eat();
                }
            });
        }
    });
    // Output their thoughts
    for thought in rx {
        println!("{thought}");
    }
}
