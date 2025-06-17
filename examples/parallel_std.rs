fn main() {
    let limit = 100_000;
    let primes = code::parallel_std::generate_primes_parallel(limit);
    println!("Found {} prime numbers up to {}", primes.len(), limit);
    println!("{:?}", primes);
}
