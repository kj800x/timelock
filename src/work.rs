use clap::ArgMatches;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rand::rngs::OsRng;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_core::RngCore;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
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

fn write_work(results: &Vec<ThreadResult>, target_file: &str) -> Result<bool, io::Error> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(target_file)?;

    for (_thread_index, initial_value, count, hash) in results {
        file.write_all(
            format!(
                "{}:{}:{}\n",
                hex::encode(initial_value),
                hex::encode(hash),
                count
            )
            .as_bytes(),
        )?;
    }

    return Result::Ok(true);
}

fn print_work(results: &Vec<ThreadResult>) {
    println!("");
    for (thread_index, initial_value, count, hash) in results {
        println!(
            "Thread {}: {} iterations\n\tInitial Seed: {}\n\tResult Hash: {}",
            thread_index,
            count,
            hex::encode(initial_value),
            hex::encode(hash)
        );
    }
}

pub fn work(work_matches: &ArgMatches) {
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

    write_work(&results, output)
        .or_else(write_work_panic)
        .unwrap();
    print_work(&results);
}
