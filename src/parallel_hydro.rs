use std::time::Duration;
use hydro_lang::*;
use crate::parallel_std;

pub struct Leader {}
pub struct Worker {}

pub fn generate_primes<'a>(leader: &Process<'a, Leader>, workers: &Cluster<'a, Worker>) {
    let primes = leader
        .source_iter(q!(2..100_000))
        .round_robin_bincode(workers)
        .filter(q!(|n| parallel_std::is_prime(*n)))
        .send_bincode_anonymous(leader)
        .fold_commutative(q!(|| Vec::new()), q!(|acc, n| acc.push(n)));


    /*let primes = workers
        .tick()
        .spin_batch(q!(1024))
        .map(q!(|i| 2 + i as usize))
        .filter(q!(|n| is_prime(n)))
        .fold(q!(|| Vec::new()), q!(|acc, n| acc.push(n)))
        .all_ticks();*/

    //let all_primes = primes.send_bincode_anonymous(leader);

    //let collected = all_primes.reduce_commutative(q!(|acc, primes| acc.extend(primes)));

    unsafe {
        primes.sample_every(q!(Duration::from_secs(10)))
            .for_each(q!(|primes: Vec<usize>| println!("{:?}", primes)));
    }
        
}