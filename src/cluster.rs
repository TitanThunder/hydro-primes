use hydro_lang::*;

pub struct Leader {}
pub struct Worker {}

pub fn cluster<'a>(leader: &Process<'a, Leader>, workers: &Cluster<'a, Worker>) {
    leader
        .source_iter(q!(0..4))
        .round_robin_bincode(workers)
        .inspect(q!(|n| {
            let strings = ["This ", "is ", "a ", "test."];
            let s = strings.get(*n as usize).unwrap_or(&"");
            println!("{}", s)
        }))
        .send_bincode_anonymous(leader)
        .for_each(q!(|n| {
            let strings = ["This ", "is ", "a ", "test."];
            let s = strings.get(n as usize).unwrap_or(&"");
            println!("{}", s)
        }))
}
