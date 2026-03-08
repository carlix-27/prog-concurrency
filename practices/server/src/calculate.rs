
use std::time::Instant;


pub fn calculate_with_time(i: u64) -> (f64, f64) {
    let start = Instant::now();
    let pi = calculate_pi(i);
    let duration = start.elapsed().as_secs_f64();
    (pi, duration)
}

fn calculate_pi(i: u64) -> f64 {
    let mut sum = 0.0;
    for n in 0..i {
        let term = (-1.0_f64).powi(n as i32) / (2.0 * n as f64 + 1.0);
        sum += term;
    }
    4.0 * sum
}