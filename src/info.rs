use crate::cli;
use crate::formats::puzzlefile;
use crate::formats::workfile;
use crate::hash;
use crate::types::*;
use linreg::linear_regression;
use std::path::Path;
use std::time::Duration;

pub fn decide_rate() -> f64 {
  let x: Vec<i32>;
  if cfg!(debug_assertions) {
    x = vec![
      10000, 10000, 10000, 10000, 10000, 100000, 100000, 100000, 100000,
    ];
  } else {
    x = vec![
      10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 100000, 100000, 100000,
      100000, 100000, 100000, 100000, 100000, 10000, 10000, 10000, 10000, 10000, 10000, 10000,
      10000, 10000, 10000, 100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000, 10000,
      10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 100000, 100000, 100000,
      100000, 100000, 100000, 100000, 100000,
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

  if d.as_millis() < 100 {
    return "no time at all".to_string();
  }

  return format!("about {}", f.convert(d));
}

pub fn info(args: &cli::Info) {
  println!("Calculating approximate hash rate...");

  let rate = decide_rate();

  println!("");
  println!(
    "This computer can calculate about {:.0} hashes per second",
    rate
  );

  let workfile_exists = Path::new(&args.work).exists();
  if workfile_exists {
    let work = workfile::read_work(&args.work).expect("WorkFile was in an invalid format");
    let total_count = workfile::total_count(&work);

    println!("");
    println!("The WorkFile contains the work of {} hashes", total_count);

    println!(
      "It would take {} to solve the WorkFile",
      estimate_duration(rate, total_count)
    )
  }

  let solutionfile_exists = Path::new(&args.solution).exists();
  if solutionfile_exists {
    let work = workfile::read_work(&args.solution).expect("SolutionFile was in an invalid format");
    let total_count = workfile::total_count(&work);

    println!("");
    println!(
      "The SolutionFile contains the work of {} hashes",
      total_count
    );

    println!(
      "It would take {} to solve the SolutionFile",
      estimate_duration(rate, total_count)
    )
  }

  let puzzlefile_exists = Path::new(&args.puzzle).exists();
  if puzzlefile_exists {
    let puzzle =
      puzzlefile::read_puzzle_file(&args.puzzle).expect("PuzzleFile was in an invalid format");
    let total_count = puzzlefile::total_count(puzzle);

    println!("");
    println!("The PuzzleFile contains the work of {} hashes", total_count);

    println!(
      "It would take {} to solve the PuzzleFile",
      estimate_duration(rate, total_count)
    )
  }
}
