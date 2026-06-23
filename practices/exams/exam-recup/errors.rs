// Fragmento 1 - Errores




// Fragmento 2 - Errores

struct BoundedBuffer<T> {
    data: Mutex<VecDequeue<T>>,
    cond: Condvar,
    capacity: usize
}


impl BoundedBuffer {
    fn new(capacity: usize) -> Self { 
        
    }

    pub fn put(&self) {
        let mut queue = self.capacity.lock().unwrap();
        while(queue.len() == self.capacity){
            queue = self.cond.wait(queue).unwrap();
        }
        self.cond.notify_one();
    }

    pub fn take(&self){
        

        self.cond.notify_one();
    }
}