// Fragmento 1 - Errores

fn main(){
    let data = Arc::new(Mutex::new(vec![1,2,3]));
    let sum = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for i in 0..3 {
        let data = data.clone();
        let sum = sum.clone();

        handles.push(thread::spawn(move || {
            let d = data.lock().unwrap();
            let s = &mut *sum.lock().unwrap();
            *s += d[i];
            thread::sleep(std::time::Duration::from_secs(1));
        }))
    }

    for h in handles { h.join().unwrap(); }
    println!("sum {} ", *sum.lock().unwrap();)
}


// Fragmento 2 - Errores

struct BoundedBuffer<T> {
    data: Mutex<VecDequeue<T>>,
    cond: Condvar,
    capacity: usize
}


impl BoundedBuffer {
    fn new(capacity: usize) -> Self {


    }

    pub fn put(&self, item: T) {
        let mut queue = self.capacity.lock().unwrap();
        while queue.len() == self.capacity {
            queue = self.cond.wait(queue).unwrap();
        }
        queue.push_back(item);
        self.cond.notify_one();
    }

    pub fn take(&self) -> T {
        let mut queue = self.data.lock().unwrap();
        while queue.len() == self.capacity {
            queue = self.cond.wait(queue).unwrap();
        }
        let item = queue.pop_front().unwrap();
        self.cond.notify_one();
        item
    }
}


// Fragmento 3 - Errores

fn main() {
    let mutex_a = Arc::new(Mutex::new(0));
    let mutex_b = Arc::new(Mutex::new(0));

    let a = mutex_a.clone();
    let b = mutex_b.clone();

    let t1 = thread::spawn(move || {
        let _ga = a.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let _gb = b.lock().unwrap();
        println!("t1 listo");
    });

    let a = mutex_a.clone();
    let b = mutex_b.clone();

    let t2 = thread::spawn(move || {
        let _gb = b.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let _ga = a.lock().unwrap();
        println!("t2 listo");
    });

    t1.join().unwrap();
    t2.join().unwrap();
}


// Fragmento 4 - Errores

struct SafeQueue<T> {
    data: Mutex<VecDeque<T>>,
    cond: Condvar,
}

impl<T> SafeQueue<T> {
    fn enqueue(&self, val: T) {
        let mut queue = self.data.lock().unwrap();
        queue.push_back(val);
        self.cond.notify_one();
    }

    fn dequeue(&self) -> T {
        let mut queue = self.data.lock().unwrap();
        if queue.is_empty() {
            queue = self.cond.wait(queue).unwrap();
        }
        queue.pop_front().unwrap()
    }
}


// Fragmento 5 - Errores

fn main() {
    let (tx, rx) = mpsc::sync_channel::<i32>(2);

    for i in 0..5 {
        tx.send(i).unwrap();
    }

    let handle = thread::spawn(move || {
        for _ in 0..5 {
            println!("{}", rx.recv().unwrap());
        }
    });

    handle.join().unwrap();
}
