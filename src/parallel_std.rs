use std::thread;

pub fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as usize;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn generate_primes_parallel(limit: usize) -> Vec<usize> {
    let num_threads = 8; // Adjust based on your CPU
    let chunk_size = (limit - 1) / num_threads + 1;

    let mut handles = vec![];

    for i in 0..num_threads {
        let start = 2 + i * chunk_size;
        let end = ((i + 1) * chunk_size + 1).min(limit + 1);

        let handle = thread::spawn(move || {
            (start..end)
                .filter(|&n| is_prime(n))
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

    primes.sort(); // Optional: ensure result is sorted
    primes
}
