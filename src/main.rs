use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rand::rngs::OsRng;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_core::RngCore;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn hash(iv: [u8; 32], stopped: &AtomicBool) -> ([u8; 32], u64) {
    let mut sha = Sha256::new();
    let mut bytes = iv;
    let mut i = 0;
    loop {
        if stopped.load(Ordering::Relaxed) {
            break;
        }
        sha.input(&bytes);
        sha.result(&mut bytes);
        sha.reset();
        i = i + 1;
    }
    return (bytes, i);
}

fn main() {
    let stopped = Arc::new(AtomicBool::new(false));
    let stopped1 = Arc::clone(&stopped);

    const THREADS: u8 = 12;

    ctrlc::set_handler(move || {
        stopped.swap(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    println!("Hello, world!");

    let mut seed = [0u8; 32];
    OsRng.fill_bytes(&mut seed);
    let mut rng = ChaCha20Rng::from_seed(seed);

    let mut join_handles: Vec<thread::JoinHandle<(u8, [u8; 32], u64, [u8; 32])>> = Vec::new();

    for thread_index in 0..THREADS {
        let shared1 = Arc::clone(&stopped1);
        let mut initial_value = [0u8; 32];
        rng.fill(&mut initial_value);
        join_handles.push(thread::spawn(move || {
            let (hash, count) = hash(initial_value, &shared1);
            (thread_index, initial_value, count, hash)
        }))
    }

    println!("");
    for join in join_handles {
        let (thread_index, initial_value, count, hash) =
            join.join().expect("Error joining the threads");

        println!(
            "Thread {}: {} iterations\n\tInitial Seed: {:x?}\n\tResult Hash: {:x?}\n",
            thread_index, count, initial_value, hash
        );
    }
}
