pub fn generate_primes_sequential(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return vec![];
    }

    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let sqrt_limit = (limit as f64).sqrt() as usize;
    for num in 2..=sqrt_limit {
        if is_prime[num] {
            for multiple in (num * num..=limit).step_by(num) {
                is_prime[multiple] = false;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(index, &prime)| if prime { Some(index) } else { None })
        .collect()
}
