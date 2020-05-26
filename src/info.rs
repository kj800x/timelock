use crate::core::*;
use crate::hash;
use crate::puzzlefile;
use crate::workfile;
use clap::ArgMatches;
use linreg::linear_regression;
use std::path::Path;
use std::time::Duration;

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

    let (slope, _intercept): (f64, f64) = linear_regression(&y, &x)
        .expect("Failed to evaluate linear regression when deciding computation power");
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
    let work_path = info_matches.value_of("work").unwrap(); // Safe because defaulted in yaml
    let puzzle_path = info_matches.value_of("puzzle").unwrap(); // Safe because defaulted in yaml

    println!("Calculating approximate hash rate...");

    let rate = decide_rate();

    println!("");
    println!(
        "This computer can calculate about {:.0} hashes per second",
        rate
    );

    let workfile_exists = Path::new(work_path).exists();
    if workfile_exists {
        let work = workfile::read_work(work_path).expect("Workfile was in an invalid format");
        let total_count = workfile::total_count(&work);

        println!("");
        println!("The workfile contains the work of {} hashes", total_count);

        println!(
            "It would take about {} to solve the workfile",
            estimate_duration(rate, total_count)
        )
    }

    let puzzlefile_exists = Path::new(puzzle_path).exists();
    if puzzlefile_exists {
        let puzzle =
            puzzlefile::read_puzzle(puzzle_path).expect("Puzzlefile was in an invalid format");
        let total_count = puzzlefile::total_count(puzzle);

        println!("");
        println!("The puzzlefile contains the work of {} hashes", total_count);

        println!(
            "It would take about {} to solve the puzzlefile",
            estimate_duration(rate, total_count)
        )
    }
}
