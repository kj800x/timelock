use crate::core::*;
use crate::hex_utils;
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

pub fn read_puzzle(target_file: &str) -> Result<Puzzle, io::Error> {
  let file = File::open(target_file)?;
  let lines = io::BufReader::new(file)
    .lines()
    .map(|line| line.expect("Failed to read a line from the puzzlefile"));
  let mut puzzle: Vec<PuzzlePiece> = Vec::new();
  for line in lines {
    let parts: Vec<&str> = line.split(':').collect();
    let seed: [u8; 32] =
      hex_utils::read_bytes(parts[0]).expect("Puzzlefile was in invalid format (seed)");
    let count: u64 = parts[1]
      .parse()
      .expect("Puzzlefile was in invalid format (count)");
    puzzle.push((seed, count));
  }
  return Ok(puzzle);
}

pub fn total_count(puzzle: Puzzle) -> u64 {
  puzzle.iter().map(|piece| piece.1).sum()
}
