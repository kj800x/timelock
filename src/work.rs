use crate::core::*;
use crate::hash;
use crate::workfile;
use clap::ArgMatches;
use rand::rngs::OsRng;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_core::RngCore;
use std::io;
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
    println!("Work is being generated... Press CTRL+C to stop and save progress.");

    let output = matches.value_of("work").unwrap(); // Safe because defaulted in yaml
    let threads: u8 = matches
        .value_of("parallelism")
        .unwrap() // Safe because defaulted in yaml
        .parse()
        .expect("Parallelism argument must be an integer");
    let target: Count = matches
        .value_of("target")
        .unwrap() // Safe because defaulted in yaml
        .parse()
        .expect("Work target argument must be an integer");
    let chain_length: Count = matches
        .value_of("chain-length")
        .unwrap() // Safe because defaulted in yaml
        .parse()
        .expect("Chain-length argument must be an integer");

    let results = generate_work(threads, target, chain_length);

    fn handle_write_error(err: io::Error) -> Result<(), io::Error> {
        println!("{:?}", err);
        println!("Error writing work! You must manually construct the workfile");
        Ok(())
    }

    workfile::write_work(&results, true, output)
        .or_else(handle_write_error)
        .unwrap(); // Safe because of the or_else

    print_work(&results);
}
