use crate::utils;
use std::thread;

pub fn generate_primes_parallel(limit: usize) -> Vec<usize> {
    let num_threads = 8; // Adjust based on your CPU
    let chunk_size = (limit - 1) / num_threads + 1;

    let mut handles = vec![];

    for i in 0..num_threads {
        let start = 2 + i * chunk_size;
        let end = ((i + 1) * chunk_size + 1).min(limit + 1);

        let handle = thread::spawn(move || {
            (start..end)
                .filter(|&n| utils::is_prime(n))
                .collect::<Vec<usize>>()
        });

        handles.push(handle);
    }

    let mut primes = vec![];
    for handle in handles {
        if let Ok(mut result) = handle.join() {
            primes.append(&mut result);
        }
    }

    primes.sort();
    primes
}
