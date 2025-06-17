use std::time::Duration;
use hydro_lang::*;
use crate::parallel_hydro;
//use crate::parallel_std;

pub struct Leader {}
pub struct Worker {}

pub fn generate_primes<'a>(leader: &Process<'a, Leader>, workers: &Cluster<'a, Worker>) {
    /*let primes = workers
        .tick()  // Ongoing ticker, similar to your "spin_batch"
        .spin_batch(q!(1024))  // Parallel worker batch size, you can tweak
        .map(q!(|_| rand::random::<u64>() + 2))  // Generate numbers, starting from 2
        .filter(q!(|n| parallel_hydro::is_prime(*n))) // Keep only prime numbers
        .fold(q!(|| Vec::new()), q!(|acc, n| {
            acc.push(n);
        }))
        .all_ticks();

    let all_primes = primes.send_bincode_anonymous(leader); // send primes to leader

    let total_primes = all_primes
        .reduce_commutative(q!(|all, from_worker| {
            all.extend(from_worker);
        }));

    unsafe {
        total_primes
            .sample_every(q!(Duration::from_secs(1)))  // Print every 10 seconds, adjust as needed
    }
        .for_each(q!(|primes| {
        println!("Found {} prime numbers", primes.len());
        // Optionally, you can print some of the primes to verify
        // println!("{:?}", &primes[0..10]); // Example: Print first 10 primes
    }));*/
    /*
    let primes = workers
        .tick()  // Ongoing ticker, similar to your "spin_batch"
        .spin_batch(q!(1024))  // Parallel worker batch size, you can tweak
        .map(q!(|_| rand::random::<u64>() + 2))
        .inspect(q!(|n| println!("{}", n)))// Generate numbers, starting from 2
        .filter(q!(|n| parallel_hydro::is_prime(*n)))
        .all_ticks()// Keep only prime numbers
        .send_bincode_anonymous(leader)
        .fold_commutative(q!(|| Vec::new()), q!(|acc, n| acc.push(n)));

    unsafe {
        primes.sample_every(q!(Duration::from_secs(1)))
            .for_each(q!(|primes| println!("Found {} prime numbers", primes.len())));
    }



    let primes = leader
        .source_iter(q!(2..)) // Unendliche Reihe von Zahlen
        .round_robin_bincode(workers)
        .filter(q!(|n| parallel_hydro::is_prime(*n)))
        .send_bincode_anonymous(leader)
        .fold_commutative(q!(|| Vec::new()), q!(|acc, n| acc.push(n)));

    // Sample alle 1 Sekunde
    unsafe {
        primes.sample_every(q!(Duration::from_secs(1)))
            .for_each(q!(|primes| {
            println!("Found {} prime numbers", primes.len());
        }));
    }
    */
    static mut CURRENT: u64 = 2;

    unsafe {
        // Wir erzeugen eine fortlaufende Zahlensequenz, die jede Sekunde eine neue Zahl liefert
        let primes = workers
            .tick() // startet einen Timer für regelmäßige Berechnungen
            .spin_batch(q!(1024)) // jeder Tick generiert eine neue Zahl
            .map(q!(|_| {
                static mut CURRENT: u64 = 2;
                let result = unsafe { CURRENT };
                unsafe { CURRENT += 1 }; // erhöht die Zahl nach jedem Tick
                result
            }))
            .filter(q!(|n| parallel_hydro::is_prime(*n))) // prüft, ob die Zahl eine Primzahl ist
            .all_ticks()
            .send_bincode_anonymous(leader)
            .fold_commutative(q!(|| Vec::new()), q!(|acc, n| acc.push(n))); // aggregiert die gefundenen Primzahlen


    // Ausgabe der gefundenen Primzahlen alle 1 SekundeHy

        primes.sample_every(q!(Duration::from_secs(1)))
            .for_each(q!(|primes: Vec<u64>| {
            println!("Found {} prime numbers, largest prime: {:?}", primes.len(), primes.iter().max().unwrap());
        }));
    }

}

pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
