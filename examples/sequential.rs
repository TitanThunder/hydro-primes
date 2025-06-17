fn main() {
    let limit = 100;
    let primes = code::sequential::generate_primes_sequential(limit);
    println!("Primes up to {}: {:?}", limit, primes);
}
