fn main() {
    let limit = 100;
    let primes = code::sequential::sieve_of_eratosthenes(limit);
    println!("Primes up to {}: {:?}", limit, primes);
}