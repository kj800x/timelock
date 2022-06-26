use crate::types::*;
use crate::utils::hex_utils;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::BufRead;

pub fn write_puzzle(puzzle: &Puzzle, target: &mut dyn Write) -> Result<(), io::Error> {
  for (hash, count) in puzzle {
    target.write_all(format!("{}:{}\n", hex::encode(hash), count).as_bytes())?;
  }

  Ok(())
}

pub fn write_puzzle_file(puzzle: &Puzzle, target_file: &str) -> Result<(), io::Error> {
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .open(target_file)?;

  write_puzzle(puzzle, &mut file)
}

pub fn read_puzzle(target: &mut dyn BufRead) -> Result<Puzzle, io::Error> {
  let lines = target
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

pub fn read_puzzle_file(target_file: &str) -> Result<Puzzle, io::Error> {
  let file = File::open(target_file)?;
  let mut lines = io::BufReader::new(file);

  read_puzzle(&mut lines)
}

pub fn total_count(puzzle: Puzzle) -> u64 {
  puzzle.iter().map(|piece| piece.1).sum()
}
