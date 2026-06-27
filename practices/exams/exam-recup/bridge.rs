// Ejercicio de un puente de un solo "cruce"
/*
Hay un puente por el que pasan una cantidad MAX de vehiculos.
- Autos llegan desde el Norte y el Sur.

Casos a tener en cuenta:
- Solo pueden pasar autos de una sola direccion en primera instancia
- De esa forma evitamos que autos "choquen"

La idea es definir el metodo enter y exit segun ese contexto.

*/

use std::sync::{Mutex, Condvar};

#[derive(Clone, Copy, PartialEq)]
enum Direction { North, South }

struct BridgeState {
    current_dir: Option<Direction>,
    count: u32,
}

pub struct Bridge {
    state: Mutex<BridgeState>,
    cond: Condvar,
    max: u32,
}

impl Bridge {
    fn new(max: u32) -> Bridge {
        Bridge {
            state: Mutex::new(BridgeState {
                current_dir: None,
                count: 0,
            }),
            cond: Condvar::new(),
            max,
        }
    }

    pub fn enter(&self, dir: Direction) {
        let mut state = self.state.lock().unwrap();

        // Espera si hay autos del sentido contrario, o si el puente está lleno
        state = self.cond.wait_while(state, |s| {
            let wrong_dir = s.current_dir.map_or(false, |d| d != dir);
            wrong_dir || s.count == self.max
        }).unwrap();

        state.current_dir = Some(dir);
        state.count += 1;
    }

    pub fn exit(&self) {
        let mut state = self.state.lock().unwrap();
        state.count -= 1;

        if state.count == 0 {
            // Último vehículo: libera la dirección para que el otro lado pueda entrar
            state.current_dir = None;
            self.cond.notify_all();
        }
    }
}
