extern crate rand;

use rand::{ChaChaRng, Rng, SeedableRng};
use std::thread;
use std::sync::mpsc;
use std::collections::HashSet;

const THREADS: usize = 4;
const SAMPLES: usize = 100_000_000;

fn get_seeds(initial_seed: &[u32]) -> HashSet<[u32; 4]> {
    let mut rng = ChaChaRng::from_seed(initial_seed);
    let seeds: HashSet<_> = (0..THREADS).map(|_| rng.gen()).collect();
    if seeds.len() != THREADS {
        panic!("There are identical seeds!\n{:?}", seeds);
    }
    seeds
}

fn generate_data(seed: [u32; 4], tx: mpsc::Sender<Vec<u32>>) {
    let mut rng = ChaChaRng::from_seed(&seed);
    let mut data = Vec::with_capacity(SAMPLES);
    for _ in 0..SAMPLES {
        data.push(rng.next_u32());
    }
    tx.send(data).unwrap();
}

fn check_duplicates(rx: mpsc::Receiver<Vec<u32>>) -> usize {
    let mut total = HashSet::with_capacity(THREADS * SAMPLES);
    rx.into_iter()
        .take(THREADS)
        .flat_map(|data| data.into_iter())
        .fold(0, |acc, i| acc + if total.insert(i) { 0 } else { 1 })
}

fn main() {
    let seeds = get_seeds(&[913453253u32]);

    let (tx, rx) = mpsc::channel();
    for seed in seeds {
        let tx = tx.clone();
        thread::spawn(move || generate_data(seed, tx));
    }
    let duplicates = check_duplicates(rx);
    println!(
        "There are {} duplicate(s), which is {}%",
        duplicates,
        100f64 * duplicates as f64 / ((THREADS * SAMPLES) as f64)
    );
}
