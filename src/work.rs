use crate::workfile;
use clap::ArgMatches;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rand::rngs::OsRng;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_core::RngCore;
use std::io;
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

fn generate_work(threads: u8) -> Vec<workfile::ThreadResult> {
    let stopped = Arc::new(AtomicBool::new(false));
    let handler_stopped_flag = Arc::clone(&stopped);

    ctrlc::set_handler(move || {
        handler_stopped_flag.swap(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    let mut seed = [0u8; 32];
    OsRng.fill_bytes(&mut seed);
    let mut rng = ChaCha20Rng::from_seed(seed);

    let mut join_handles: Vec<thread::JoinHandle<workfile::ThreadResult>> = Vec::new();

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

fn print_work(results: &Vec<workfile::ThreadResult>) {
    println!("");
    for (thread_index, initial_value, count, hash) in results {
        println!(
            "Thread {}: {} iterations\n\tInitial Seed: {}\n\tResult Hash : {}",
            thread_index,
            count,
            hex::encode(initial_value),
            hex::encode(hash)
        );
    }
}

pub fn work(work_matches: &ArgMatches) {
    println!("Work is being generated... Press CTRL+C to stop work and save to the workfile.");

    let output = work_matches.value_of("OUTPUT").unwrap(); // required
    let threads: u8 = work_matches
        .value_of("parallelism")
        .unwrap() // defaulted
        .parse()
        .expect("Parallelism argument must be an integer");

    let results = generate_work(threads);

    fn write_work_panic(_: io::Error) -> Result<bool, io::Error> {
        println!("Error writing work! You must manually construct the workfile");
        Result::Ok(true)
    }

    workfile::write_work(&results, output)
        .or_else(write_work_panic)
        .unwrap();
    print_work(&results);
}
