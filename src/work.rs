use crate::core::*;
use crate::hash;
use crate::time;
use crate::workfile;
use clap::ArgMatches;
use rand::rngs::OsRng;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_core::RngCore;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

fn generate_work(threads: u8, target: Count, chain_length: Count) -> Work {
    let root_stop_flag = Arc::new(AtomicBool::new(false));
    let handler_stop_flag = Arc::clone(&root_stop_flag);

    let (sender, receiver) = channel::<Chain>();

    ctrlc::set_handler(move || {
        handler_stop_flag.swap(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    let mut seed = [0u8; 32];
    OsRng.fill_bytes(&mut seed);
    let mut rng = ChaCha20Rng::from_seed(seed);

    let mut join_handles: Vec<thread::JoinHandle<()>> = Vec::new();

    for _ in 0..threads {
        let thread_max_count = match chain_length {
            0 => target / (threads as u64),
            _ => chain_length,
        };
        let thread_stop_flag = Arc::clone(&root_stop_flag);
        let thread_sender = sender.clone();
        let mut iv = [0u8; 32];
        rng.fill(&mut iv);
        join_handles.push(thread::spawn(move || loop {
            if thread_stop_flag.load(Ordering::Relaxed) {
                return;
            }
            thread_sender
                .send(hash::hash(iv, thread_max_count, &thread_stop_flag))
                .expect("Expected main thread to be ready to receive values");
        }))
    }

    std::mem::drop(sender);

    let mut work: Work = Vec::new();
    let mut total_work_made: u64 = 0;

    loop {
        match receiver.recv() {
            Ok(chain) => {
                work.push(chain);
                total_work_made = total_work_made + chain.1;
                if target != 0 && total_work_made >= target {
                    root_stop_flag.swap(true, Ordering::Relaxed);
                }
            }
            Err(_) => break,
        }
    }

    for handle in join_handles {
        handle.join().expect("Error joining the threads")
    }

    work
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
    let output = matches.value_of("work").unwrap(); // Safe because defaulted in yaml

    // Test to see if we can write the output file before we generate the work
    // It is safe to write an empty vector to the file, since we're working in append mode
    workfile::write_work(&Vec::new(), true, output)
        .expect("Refusing to do work - Unable to write to target file");

    let threads: u8 = matches
        .value_of("parallelism")
        .unwrap() // Safe because defaulted in yaml
        .parse()
        .expect("Parallelism argument must be an integer");
    let target: Count = time::parse_time(matches.value_of("target").unwrap()); // Safe because defaulted in yaml
    let chain_length: Count = time::parse_time(matches.value_of("chain-length").unwrap()); // Safe because defaulted in yaml

    println!("Work is being generated... Press CTRL+C to stop and save progress.");

    let results = generate_work(threads, target, chain_length);

    workfile::write_work(&results, true, output)
        .expect("Sorry, unable to write work. Please report an issue and include this error");

    print_work(&results);
}
