use std::convert::TryInto;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::BufRead;

pub type ThreadResult = (u8, [u8; 32], u64, [u8; 32]);

pub fn write_work(results: &Vec<ThreadResult>, target_file: &str) -> Result<bool, io::Error> {
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

fn read_bytes(byte_str: &str) -> [u8; 32] {
  let vector = hex::decode(byte_str).unwrap();

  let mut arr = [0u8; 32];
  for (place, element) in arr.iter_mut().zip(vector.iter()) {
    *place = *element;
  }
  arr
}

pub fn read_work(target_file: &str) -> Result<Vec<ThreadResult>, io::Error> {
  let file = File::open(target_file)?;
  let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
  let mut results: Vec<ThreadResult> = Vec::new();
  for (i, line) in lines.enumerate() {
    let parts: Vec<&str> = line.split(':').collect();
    let seed: [u8; 32] = read_bytes(parts[0]);
    let hash: [u8; 32] = read_bytes(parts[1]);
    let count: u64 = parts[2].parse().unwrap();
    results.push((i.try_into().unwrap(), seed, count, hash));
  }
  return Ok(results);
}
