// Solucion alternativa usando solo `std::sync::mpsc`.
//
// La idea clave: `sync_channel(capacidad)` ES la sala de espera.
// - La capacidad acotada del canal modela directamente las sillas.
// - `try_send` no bloquea: si el canal esta lleno, el cliente se va (no hay
//   sillas libres), igual que en el enunciado original.
// - El barbero hace `recv()` (via el iterador del receiver), que bloquea su
//   hilo cuando no hay clientes: asi se modela que "duerme".
// - Cada cliente viaja con su propio canal de "vuelta" (`done`) para que el
//   barbero le avise cuando termino su corte, sin necesidad de un estado
//   compartido adicional ni de comparar ids a mano.

use std::sync::mpsc::{self, SyncSender, TrySendError};
use std::thread;
use std::time::Duration;

const NUM_CHAIRS: usize = 3;
const NUM_CUSTOMERS: usize = 10;

struct Customer {
    id: u32,
    done: mpsc::Sender<()>,
}

fn barber(shop_rx: mpsc::Receiver<Customer>) {
    for customer in shop_rx {
        println!("Barbero corta el pelo de cliente {}", customer.id);
        thread::sleep(Duration::from_millis(50));
        let _ = customer.done.send(());
    }
}

fn customer_arrives(id: u32, shop_tx: &SyncSender<Customer>) {
    let (done_tx, done_rx) = mpsc::channel();
    match shop_tx.try_send(Customer { id, done: done_tx }) {
        Ok(()) => {
            println!("Cliente {id} se sienta a esperar");
            done_rx.recv().unwrap();
            println!("Cliente {id}: corte terminado, se va satisfecho");
        }
        Err(TrySendError::Full(_)) => {
            println!("Cliente {id}: sala llena, se va");
        }
        Err(TrySendError::Disconnected(_)) => unreachable!("el barbero nunca cierra el local"),
    }
}

fn main() {
    let (shop_tx, shop_rx) = mpsc::sync_channel::<Customer>(NUM_CHAIRS);

    thread::spawn(move || barber(shop_rx));

    let handles: Vec<_> = (0..NUM_CUSTOMERS)
        .map(|id| {
            let shop_tx = shop_tx.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(id as u64 * 15));
                customer_arrives(id as u32, &shop_tx);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
