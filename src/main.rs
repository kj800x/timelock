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

type ThreadResult = (u8, [u8; 32], u64, [u8; 32]);

fn generate_work(threads: u8) -> Vec<ThreadResult> {
    let stopped = Arc::new(AtomicBool::new(false));
    let handler_stopped_flag = Arc::clone(&stopped);

    ctrlc::set_handler(move || {
        handler_stopped_flag.swap(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    let mut seed = [0u8; 32];
    OsRng.fill_bytes(&mut seed);
    let mut rng = ChaCha20Rng::from_seed(seed);

    let mut join_handles: Vec<thread::JoinHandle<ThreadResult>> = Vec::new();

    for thread_index in 0..threads {
        let thread_stopped_flag = Arc::clone(&stopped);
        let mut initial_value = [0u8; 32];
        rng.fill(&mut initial_value);
        join_handles.push(thread::spawn(move || {
            let (hash, count) = hash(initial_value, &thread_stopped_flag);
            (thread_index, initial_value, count, hash)
        }))
    }

    join_handles
        .into_iter()
        .map(|join_handle| join_handle.join().expect("Error joining the threads"))
        .collect()
}

fn print_results(results: Vec<ThreadResult>) {
    println!("");
    for (thread_index, initial_value, count, hash) in results {
        println!(
            "Thread {}: {} iterations\n\tInitial Seed: {:x?}\n\tResult Hash: {:x?}",
            thread_index, count, initial_value, hash
        );
    }
}

fn main() {
    let results = generate_work(8);
    print_results(results);
}
