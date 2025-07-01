fn main() {
    let limit = 100_000;
    let primes = code::parallel_std::generate_primes_parallel(limit);
    println!("Primes up to {}: {:?}", limit, primes);
    println!("Found {} primes.", primes.len());
}
