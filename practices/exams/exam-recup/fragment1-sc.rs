// Fragmento 1 - Original

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
    

// Cual es el error en este codigo? 

/*



*/