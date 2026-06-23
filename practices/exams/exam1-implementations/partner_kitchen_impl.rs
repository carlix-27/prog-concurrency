// Codigo que usa el colega

pub struct Kitchen {
    burners: Mutex<Vec<i32>>,
    boards: Mutex<Vec<i32>>
}

impl Kitchen {
    pub fn take_burner(&self) -> u32 {
        loop {
            if let Some(b) = self.burners.lock().unwrap().pop() { return b; }
        }
    }

    pub fn take_board(&self) -> u32 {
        loop {
            if let Some(b) = self.boards.lock().unwrap().pop() { return b; }
        }
    }
}

// Y en el main

// Cocinero: necesita ambos
let t = kitchen.take_board();
let b = kitchen.take_burner();
// ...cocina...
kitchen.release_board(t);
kitchen.release_burner(b);


// Cortador: solo necesita tabla
let t = kitchen.take_board();
// ...corta...
kitchen.release_burner(b);