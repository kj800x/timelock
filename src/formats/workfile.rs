use crate::core::*;
use crate::hex_utils;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::BufRead;

pub fn write_work(work: &Work, append: bool, target_file: &str) -> Result<(), io::Error> {
  let mut file = OpenOptions::new()
    .append(append)
    .write(!append)
    .truncate(!append)
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

pub fn total_count(work: &Work) -> u64 {
  work.iter().map(|chain| chain.1).sum()
}

/** Split a WorkFile such that the first returned WorkFile in the tuple has at least the given amount of work */
pub fn split(mut work: Work, amount: Count) -> (Work, Work) {
  // 0 means max, so return the entire workfile as the SolutionFile and return an empty vector for the new WorkFile
  if amount == 0 {
    return (work, Vec::new());
  }

  work.sort_by(|chain_a, chain_b| chain_a.1.partial_cmp(&chain_b.1).unwrap());

  let mut out_solution: Work = Vec::new();
  let mut out_work: Work = Vec::new();
  let mut acc = 0;

  for chain in work {
    let hashes = chain.1;
    if acc < amount {
      out_solution.push(chain);
      acc = acc + hashes;
    } else {
      out_work.push(chain);
    }
  }

  return (out_solution, out_work);
}

pub fn read_work(target_file: &str) -> Result<Work, io::Error> {
  let file = File::open(target_file)?;
  let lines = io::BufReader::new(file)
    .lines()
    .map(|l| l.expect("Failed to read a line from the workfile"));
  let mut results: Vec<Chain> = Vec::new();
  for line in lines {
    let parts: Vec<&str> = line.split(':').collect();
    let seed: [u8; 32] =
      hex_utils::read_bytes(parts[0]).expect("Workfile was in invalid format (iv)");
    let hash: [u8; 32] =
      hex_utils::read_bytes(parts[1]).expect("Workfile was in invalid format (hash)");
    let count: u64 = parts[2]
      .parse()
      .expect("Workfile was in invalid format (count)");
    results.push((seed, count, hash));
  }
  return Ok(results);
}
