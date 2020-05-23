use crate::workfile;
use clap::ArgMatches;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rand::rngs::OsRng;
use rand_core::RngCore;
use std::time::Instant;

use linreg::linear_regression;

fn time_hash(count: i32) -> std::time::Duration {
    let mut sha = Sha256::new();
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    let start = Instant::now();

    for _ in 0..count {
        sha.input(&bytes);
        sha.result(&mut bytes);
        sha.reset();
    }

    let end = Instant::now();
    end.duration_since(start)
}

fn decide_rate() -> f64 {
    let x: Vec<i32>;
    if cfg!(debug_assertions) {
        x = vec![
            10000, 10000, 10000, 10000, 10000, 100000, 100000, 100000, 100000,
        ];
    } else {
        x = vec![
            10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 100000, 100000,
            100000, 100000, 100000, 100000, 100000, 100000, 10000, 10000, 10000, 10000, 10000,
            10000, 10000, 10000, 10000, 10000, 100000, 100000, 100000, 100000, 100000, 100000,
            100000, 100000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000,
            100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000,
        ];
    }

    let mut y: Vec<f64> = Vec::new();
    for count in &x {
        let duration = time_hash(*count);
        y.push(duration.as_secs_f64());
    }

    let (slope, _intercept): (f64, f64) = linear_regression(&y, &x).unwrap();
    slope
}

pub fn info(info_matches: &ArgMatches) {
    println!("Calculating approximate hash rate...");
    let slope = decide_rate();

    println!(
        "Under current conditions, this computer can calculate {:.0} hashes per second",
        slope
    );

    let input = info_matches.value_of("INPUT").unwrap();

    let workfile_exists = std::path::Path::new(input).exists();
    if workfile_exists {
        let work = workfile::read_work(input).unwrap();
        let total_count: u64 = work.iter().map(|result| result.2).sum();

        println!(
            "The current workfile contains the work of {} hashes",
            total_count
        );

        let mut f = timeago::Formatter::new();
        f.ago("");
        let d = std::time::Duration::from_secs(((total_count as f64) / slope) as u64);

        println!(
            "It would take about {} to solve the current workfile",
            f.convert(d)
        )
    }
}
