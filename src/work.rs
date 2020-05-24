use crate::core::*;
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

fn hash(iv: Hash, stopped: &AtomicBool) -> Chain {
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
    return (iv, i, bytes);
}

fn generate_work(threads: u8) -> Work {
    let root_stop_flag = Arc::new(AtomicBool::new(false));
    let handler_stop_flag = Arc::clone(&root_stop_flag);

    ctrlc::set_handler(move || {
        handler_stop_flag.swap(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    let mut seed = [0u8; 32];
    OsRng.fill_bytes(&mut seed);
    let mut rng = ChaCha20Rng::from_seed(seed);

    let mut join_handles: Vec<thread::JoinHandle<Chain>> = Vec::new();

    for _ in 0..threads {
        let thread_stop_flag = Arc::clone(&root_stop_flag);
        let mut iv = [0u8; 32];
        rng.fill(&mut iv);
        join_handles.push(thread::spawn(move || hash(iv, &thread_stop_flag)))
    }

    join_handles
        .into_iter()
        .map(|join_handle| join_handle.join().expect("Error joining the threads"))
        .collect()
}

fn print_work(work: &Work) {
    println!("");
    for (chain_index, (iv, count, hash)) in work.iter().enumerate() {
        println!(
            "Chain {}: {} iterations\n\tInitial Seed: {}\n\tResult Hash : {}",
            chain_index,
            count,
            hex::encode(iv),
            hex::encode(hash)
        );
    }
}

pub fn work(matches: &ArgMatches) {
    println!("Work is being generated... Press CTRL+C to stop and save progress.");

    let output = matches.value_of("OUTPUT").unwrap(); // required
    let threads: u8 = matches
        .value_of("parallelism")
        .unwrap() // defaulted
        .parse()
        .expect("Parallelism argument must be an integer");

    let results = generate_work(threads);

    fn write_work_panic(err: io::Error) -> Result<(), io::Error> {
        println!("{:?}", err);
        println!("Error writing work! You must manually construct the workfile");
        Ok(())
    }

    workfile::write_work(&results, output)
        .or_else(write_work_panic)
        .unwrap();

    print_work(&results);
}
