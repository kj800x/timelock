use crate::core::*;
use crate::hex_utils;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::BufRead;

pub fn write_work(work: &Work, target_file: &str) -> Result<(), io::Error> {
  let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(target_file)?;

  for (initial_value, count, hash) in work {
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

  Ok(())
}

pub fn total_count(work: Work) -> u64 {
  work.iter().map(|chain| chain.1).sum()
}

pub fn read_work(target_file: &str) -> Result<Work, io::Error> {
  let file = File::open(target_file)?;
  let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
  let mut results: Vec<Chain> = Vec::new();
  for line in lines {
    let parts: Vec<&str> = line.split(':').collect();
    let seed: [u8; 32] =
      hex_utils::read_bytes(parts[0]).expect("Work file was in invalid format (iv)");
    let hash: [u8; 32] =
      hex_utils::read_bytes(parts[1]).expect("Work file was in invalid format (hash)");
    let count: u64 = parts[2].parse().unwrap();
    results.push((seed, count, hash));
  }
  return Ok(results);
}
