use crate::utils;
use hydro_lang::*;
use std::time::Duration;

pub struct Leader {}
pub struct Worker {}

pub fn generate_primes<'a>(leader: &Process<'a, Leader>, workers: &Cluster<'a, Worker>) {
    static mut CURRENT: usize = 2;

    unsafe {
        let primes = workers
            .tick()
            .spin_batch(q!(1))
            .map(q!(|_| {
                static mut CURRENT: usize = 2;
                let result = unsafe { CURRENT };
                unsafe { CURRENT += 1 };
                result
            }))
            .filter(q!(|n| utils::is_prime(*n)))
            .all_ticks()
            .send_bincode_anonymous(leader)
            .fold_commutative(q!(|| Vec::new()), q!(|acc, n| acc.push(n)));

        primes
            .sample_every(q!(Duration::from_secs(1)))
            .for_each(q!(|primes: Vec<usize>| {
                println!("Found {} prime numbers", primes.len());
            }));
    }
}
