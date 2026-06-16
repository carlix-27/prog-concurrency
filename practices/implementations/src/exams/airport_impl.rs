
use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

pub struct Runway {
    id: u32,
    is_occupied: bool,
}

impl Runway {
    fn new(id: u32) -> Self {
        Runway {
            id,
            is_occupied: false,
        }
    }
}

pub struct Plane {
    id: u32,
}

impl Plane {
    fn new(id: u32) -> Self {
        Plane { id }
    }
}

pub struct Airport {
    runways: Mutex<Vec<Runway>>,
    runway_available: Condvar,
}

impl Airport {
    fn new(runways: Vec<Runway>) -> Self {
        Airport {
            runways: Mutex::new(runways),
            runway_available: Condvar::new(),
        }
    }

    fn request_runway(&self) -> u32 {
        let mut runways = self.runways.lock().unwrap();

        loop {
            if let Some(runway) = runways.iter_mut().find(|r| !r.is_occupied) {
                runway.is_occupied = true;
                return runway.id;
            }

            runways = self.runway_available.wait(runways).unwrap();
        }
    }

    fn release_runway(&self, runway_id: u32) {
        let mut runways = self.runways.lock().unwrap();

        if let Some(runway) = runways.iter_mut().find(|r| r.id == runway_id) {
            runway.is_occupied = false;
        }

        self.runway_available.notify_one();
    }
}

fn main() {
    let runaways = (0..3).map(|id| Runway::new(id)).collect();
    let planes = (0..10).map(|id| Plane::new(id));
    let arc_airport = Arc::new(Airport::new(runaways));
    thread::scope(|s| {
        for plane in planes {
            let airport = arc_airport.clone();
            let plane_id = plane.id;
            s.spawn(move || {
                println!("Plane {}, requesting landing", plane_id);
                let runway_id = airport.request_runway();

                println!("Plane {}, landing on runway{}", plane_id, runway_id);
                thread::sleep(Duration::from_secs(1));

                println!("Plane {}, landed", plane_id);
                airport.release_runway(runway_id);
            });
        }
    });
}
