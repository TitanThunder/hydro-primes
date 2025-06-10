use rayon::prelude::*;

fn is_prime(n: usize) -> bool {
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

fn generate_primes_parallel(limit: usize) -> Vec<usize> {
    (2..=limit)
        .into_par_iter() // parallel iterator from Rayon
        .filter(|&n| is_prime(n))
        .collect()
}

fn main() {
    let limit = 100_000;
    let primes = generate_primes_parallel(limit);
    println!("Found {} prime numbers up to {}", primes.len(), limit);
}
