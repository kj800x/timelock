use crate::cli;
use crate::formats::puzzlefile;
use crate::formats::workfile;
use crate::hash;
use crate::types::*;

fn xor(hash1: Hash, hash2: Hash) -> Hash {
  let mut hash: Hash = [0u8; 32];
  for i in 0..32 {
    hash[i] = hash1[i] ^ hash2[i];
  }
  hash
}

pub fn solve_puzzle(puzzle: Puzzle) -> Work {
  let mut last_hash: Hash = [0u8; 32];
  let mut work: Work = Vec::new();
  for (i, (xor_seed, count)) in puzzle.iter().enumerate() {
    println!(
      "Beginning to solve chain {} which is {} computations long",
      i, count
    );
    let seed = xor(*xor_seed, last_hash);
    last_hash = hash::hash_count(seed, *count);
    work.push((seed, *count, last_hash));
  }
  println!("Puzzle solved!");
  work
}

pub fn convert_to_puzzle(work: Work) -> Puzzle {
  let mut last_hash: Hash = [0u8; 32];
  let mut puzzle: Puzzle = Vec::new();
  for (iv, count, hash) in work {
    puzzle.push((xor(last_hash, iv), count));
    last_hash = hash;
  }
  puzzle
}

pub fn solve(args: &cli::Solve) {
  let puzzle = puzzlefile::read_puzzle(&args.puzzle).expect("Failed to read puzzle");
  let work = solve_puzzle(puzzle);
  workfile::write_work(&work, false, &args.solution).expect("Failed to write solution");
}

pub fn puzzle(args: &cli::Secure) {
  let work = workfile::read_work(&args.solution).expect("Failed to read workfile");
  let puzzle = convert_to_puzzle(work);
  puzzlefile::write_puzzle(&puzzle, &args.puzzle).expect("Failed to write puzzle");
}
