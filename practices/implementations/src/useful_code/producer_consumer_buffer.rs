pub struct Buffer<T> {
    buffer: Mutex<VecDequeue<T>>,
    not_empty: Condvar,
    not_full: Condvar,
    capacity: usize,
}
impl Buffer<T> {
    fn produce(&self, element: T) {
        let mut buffer = self.buffer.lock().unwrap();
        while buffer.len() >= self.capacity {
            buffer = self.not_full.wait(buffer).unwrap();
        }
        buffer.push_back(element);
        self.not_empty.notify_one();
    }
    fn consume(&self) -> T {
        let mut buffer = self.buffer.lock().unwrap();
        while buffer.len() == 0 {
            buffer = self.not_empty.wait(buffer).unwrap();
        }
        let result = buffer.pop_front().unwrap();
        self.not_full.notify_one();
        return result
    }
}