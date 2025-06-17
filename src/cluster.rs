use hydro_lang::*;

pub struct Leader {}
pub struct Worker {}

pub fn cluster<'a>(leader: &Process<'a, Leader>, workers: &Cluster<'a, Worker>) {
    leader
        .source_iter(q!([
            "ThIs", "Is", "JuSt", "A", "ShOrT", "HyDrO-", "FlOw", "TeSt."
        ]))
        .map(q!(|s| s.to_string()))
        .round_robin_bincode(workers)
        .inspect(q!(|s| println!("{:?}", s)))
        .map(q!(|s| s.to_lowercase()))
        .send_bincode(leader)
        .for_each(q!(|(_, s)| println!("{:?}", s)))
}
