use crate::core::*;
use crate::puzzlefile;
use crate::workfile;
use clap::ArgMatches;

fn xor(hash1: Hash, hash2: Hash) -> Hash {
    let mut hash: Hash = [0u8; 32];
    for i in 0..32 {
        hash[i] = hash1[i] ^ hash2[i];
    }
    hash
}

pub fn solve_puzzle(_puzzle: Puzzle) -> Work {
    Vec::new()
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

pub fn solve(solve_matches: &ArgMatches) {
    let input = solve_matches.value_of("INPUT").unwrap(); // OK because defaulted in yaml
    let output = solve_matches.value_of("OUTPUT").unwrap(); // OK because defaulted in yaml
    let puzzle = puzzlefile::read_puzzle(input).expect("Unable to read puzzle");
    let work = solve_puzzle(puzzle);
    workfile::write_work(&work, output).expect("Unable to write solution");
}

pub fn puzzle(puzzle_matches: &ArgMatches) {
    let input = puzzle_matches.value_of("INPUT").unwrap(); // OK because defaulted in yaml
    let output = puzzle_matches.value_of("OUTPUT").unwrap(); // OK because defaulted in yaml
    let work = workfile::read_work(input).expect("Unable to read workfile");
    let puzzle = convert_to_puzzle(work);
    puzzlefile::write_puzzle(&puzzle, output).expect("Unable to write puzzle");
}
