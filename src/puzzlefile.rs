use crate::core::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::BufRead;

pub fn write_puzzle(puzzle: &Puzzle, target_file: &str) -> Result<(), io::Error> {
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .open(target_file)?;

  for (hash, count) in puzzle {
    file.write_all(format!("{}:{}\n", hex::encode(hash), count).as_bytes())?;
  }

  Ok(())
}

fn read_bytes(byte_str: &str) -> Result<Hash, hex::FromHexError> {
  let mut arr = [0u8; 32];
  hex::decode_to_slice(byte_str, &mut arr)?;
  Ok(arr)
}

pub fn read_puzzle(target_file: &str) -> Result<Puzzle, io::Error> {
  let file = File::open(target_file)?;
  let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
  let mut puzzle: Vec<PuzzlePiece> = Vec::new();
  for line in lines {
    let parts: Vec<&str> = line.split(':').collect();
    let seed: [u8; 32] = read_bytes(parts[0]).expect("Puzzle file was in invalid format");
    let count: u64 = parts[1].parse().unwrap();
    puzzle.push((seed, count));
  }
  return Ok(puzzle);
}

// pub fn total_count(puzzle: Puzzle) -> u64 {
//   puzzle.iter().map(|piece| piece.1).sum()
// }
