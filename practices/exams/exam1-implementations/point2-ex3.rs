// Esqueleto de codigo que necesitamos usar

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

enum ChefType { Cook, Cutter, Reheater }

struct KitchenState {
    burners: Vec<bool>, // true = libre
    boards: Vec<bool>,  // true = libre
}

pub struct Kitchen {
    state: Mutex<KitchenState>,
    cond: Condvar,
}

impl Kitchen {
    fn new(n_burners: u32, n_boards: u32) -> Kitchen {
        Kitchen {
            state: Mutex::new(KitchenState {
                burners: vec![true; n_burners as usize],
                boards: vec![true; n_boards as usize],
            }),
            cond: Condvar::new(),
        }
    }

    pub fn take_resources(&self, chef_type: &ChefType)
        -> (Option<u32>, Option<u32>) {
        let need_burner = matches!(chef_type, ChefType::Cook | ChefType::Reheater);
        let need_board = matches!(chef_type, ChefType::Cook | ChefType::Cutter);

        let mut state = self.state.lock().unwrap();
        loop {
            let burner_idx = if need_burner { find_free(&state.burners) } else { None };
            let board_idx = if need_board { find_free(&state.boards) } else { None };

            let burner_ready = !need_burner || burner_idx.is_some();
            let board_ready = !need_board || board_idx.is_some();

            if burner_ready && board_ready {
                if let Some(i) = burner_idx { state.burners[i] = false; }
                if let Some(i) = board_idx { state.boards[i] = false; }
                return (burner_idx.map(|i| i as u32), board_idx.map(|i| i as u32));
            }

            state = self.cond.wait(state).unwrap();
        }
    }

    pub fn release_resources(&self,
        burner: Option<u32>, board: Option<u32>) {
        let mut state = self.state.lock().unwrap();
        if let Some(b) = burner { state.burners[b as usize] = true; }
        if let Some(t) = board { state.boards[t as usize] = true; }
        self.cond.notify_all();
    }
}

fn find_free(resources: &[bool]) -> Option<usize> {
    resources.iter().position(|&free| free)
}

fn main() {
    let kitchen = Arc::new(Kitchen::new(3, 2));

    let chefs = vec![
        (0, ChefType::Cook),    (1, ChefType::Cutter),
        (2, ChefType::Cook),    (3, ChefType::Reheater),
        (4, ChefType::Cutter),  (5, ChefType::Cook),
        (6, ChefType::Reheater), (7, ChefType::Cutter),
    ];

    thread::scope(|s| {
        for (chef_id, chef_type) in &chefs {
            let kitchen = kitchen.clone();
            s.spawn(move || {
                println!("Chef {} waiting", chef_id);
                let (burner, board) =
                    kitchen.take_resources(&chef_type);
                if let Some(b) = burner {
                    println!("Chef {} got burner {}", chef_id, b);
                }
                if let Some(t) = board {
                    println!("Chef {} got board {}", chef_id, t);
                }
                thread::sleep(
                    std::time::Duration::from_secs(1));

                println!("Chef {} done", chef_id);
                kitchen.release_resources(burner, board);
            });
        }
    })
}
