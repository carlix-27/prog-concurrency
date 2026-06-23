fn main(){
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..5 {
        let handle = thread::spawn({
            let mut num = counter.lock().unwrap();
            *num += 1;
        })
        handles.push(handle);
    }

    for h in handles { h.join().unwrap(); }
    println!("Result: {}", *counter.lock().unwrap());
}