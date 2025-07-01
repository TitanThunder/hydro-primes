fn main() {
    let limit = 100_000;
    let primes = code::sequential::generate_primes_sequential(limit);
    println!("Primes up to {}: {:?}", limit, primes);
    println!("Found {} primes.", primes.len());
}
