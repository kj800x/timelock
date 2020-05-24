use crate::workfile;
use clap::ArgMatches;
use std::path::Path;
use std::time::Duration;

use crate::core::*;
use crate::hash;
use linreg::linear_regression;

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
        let duration = hash::time_hash(*count);
        y.push(duration.as_secs_f64());
    }

    let (slope, _intercept): (f64, f64) = linear_regression(&y, &x).unwrap();
    slope
}

fn estimate_duration(rate: f64, total_count: Count) -> String {
    let mut f = timeago::Formatter::new();
    f.ago("");
    f.num_items(2);
    let d = Duration::from_secs(((total_count as f64) / rate) as u64);

    f.convert(d)
}

pub fn info(info_matches: &ArgMatches) {
    let input = info_matches.value_of("INPUT").unwrap();

    println!("Calculating approximate hash rate...");

    let rate = decide_rate();

    println!(
        "Under current conditions, this computer can calculate {:.0} hashes per second",
        rate
    );

    let workfile_exists = Path::new(input).exists();
    if workfile_exists {
        let work = workfile::read_work(input).unwrap();
        let total_count = workfile::total_count(work);

        println!(
            "The current workfile contains the work of {} hashes",
            total_count
        );

        println!(
            "It would take about {} to solve the current workfile",
            estimate_duration(rate, total_count)
        )
    }
}
