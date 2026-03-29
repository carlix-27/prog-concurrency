use std::thread;
use std::time::Instant;

pub fn calculate_pi_parallel(n_terms: u64, n_threads: usize) -> (f64, f64) {
    let start_time = Instant::now();

    let chunk_size = n_terms / n_threads as u64;
    let mut handles = Vec::new();

    for i in 0..n_threads {
        let start = i as u64 * chunk_size;
        let end = if i == n_threads - 1 {
            n_terms
        } else {
            start + chunk_size
        };

        let handle = thread::spawn(move || {
            let mut local_sum = 0.0;
            for n in start..end {
                let sign = if n % 2 == 0 { 1.0_f64 } else { -1.0_f64 };
                let term = sign / (2.0 * n as f64 + 1.0);
                local_sum += term;
            }
            local_sum
        });

        handles.push(handle);
    }

    let total_sum: f64 = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .sum();

    let duration = start_time.elapsed().as_secs_f64(); // ← para el cronómetro

    (4.0 * total_sum, duration)
}