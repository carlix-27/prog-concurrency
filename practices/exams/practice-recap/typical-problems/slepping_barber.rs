// Solucion "clasica": Mutex + Condvar.
//
// - `queue` + `chairs` modelan la sala de espera (cantidad acotada de sillas).
// - El barbero usa `Condvar::wait_while` para dormir cuando no hay clientes.
// - `last_served` + `haircut_done` es un "buzon" de un solo lugar que sincroniza
//   al barbero con el cliente que esta siendo atendido: el barbero espera que
//   el buzon este vacio antes de escribir, y el cliente espera a que aparezca
//   su propio id antes de irse. Esto evita perder notificaciones (wakeups
//   espureos) entre clientes distintos.

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

const NUM_CHAIRS: usize = 3;
const NUM_CUSTOMERS: usize = 10;

struct Shop {
    chairs: usize,
    queue: Mutex<VecDeque<u32>>,
    customer_arrived: Condvar,
    last_served: Mutex<Option<u32>>,
    haircut_done: Condvar,
}

impl Shop {
    fn new(chairs: usize) -> Self {
        Shop {
            chairs,
            queue: Mutex::new(VecDeque::new()),
            customer_arrived: Condvar::new(),
            last_served: Mutex::new(None),
            haircut_done: Condvar::new(),
        }
    }

    fn barber_loop(&self) {
        loop {
            let mut queue = self.queue.lock().unwrap();
            queue = self
                .customer_arrived
                .wait_while(queue, |q| q.is_empty())
                .unwrap();
            let customer = queue.pop_front().unwrap();
            println!("Barbero atiende a cliente {customer} ({} esperando)", queue.len());
            drop(queue);

            thread::sleep(Duration::from_millis(50)); // corte de pelo

            let mut last = self.last_served.lock().unwrap();
            last = self.haircut_done.wait_while(last, |l| l.is_some()).unwrap();
            *last = Some(customer);
            self.haircut_done.notify_all();
        }
    }

    /// Devuelve `true` si el cliente logro sentarse y cortarse el pelo,
    /// `false` si el local estaba lleno y se tuvo que ir.
    fn arrive(&self, id: u32) -> bool {
        {
            let mut queue = self.queue.lock().unwrap();
            if queue.len() >= self.chairs {
                println!("Cliente {id}: sala llena, se va");
                return false;
            }
            queue.push_back(id);
            println!("Cliente {id} se sienta a esperar");
            self.customer_arrived.notify_one();
        }

        let mut last = self.last_served.lock().unwrap();
        last = self
            .haircut_done
            .wait_while(last, |l| *l != Some(id))
            .unwrap();
        *last = None;
        self.haircut_done.notify_all();
        println!("Cliente {id}: corte terminado, se va satisfecho");
        true
    }
}

fn main() {
    let shop = Arc::new(Shop::new(NUM_CHAIRS));

    // El barbero duerme/trabaja en loop infinito: se lanza "suelto" (no en
    // thread::scope) para no bloquear la salida del programa una vez que
    // todos los clientes ya pasaron.
    let barber_shop = Arc::clone(&shop);
    thread::spawn(move || barber_shop.barber_loop());

    let handles: Vec<_> = (0..NUM_CUSTOMERS)
        .map(|id| {
            let shop = Arc::clone(&shop);
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(id as u64 * 15));
                shop.arrive(id as u32);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
