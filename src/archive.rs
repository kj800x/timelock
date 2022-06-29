use crate::formats::puzzlefile;
use crate::puzzle::solve_puzzle;
use crate::types::*;
use crate::{cli, puzzle};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rsa::pkcs1::EncodeRsaPublicKey;
use rsa::RsaPrivateKey;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use zip::result::ZipResult;
use zip::write::FileOptions;
use zip::ZipArchive;
use zip::ZipWriter;

fn get_solution_key(solution: &Work) -> Hash {
  solution.last().unwrap().2
}

pub fn create(args: &cli::Create) -> ZipResult<()> {
  let path = Path::new(&args.archive);

  if path.exists() {
    panic!("archive already exists, refusing to overwrite")
  }

  // Generate archive contents
  // - Puzzle
  let blank_puzzle = puzzle::new_blank_puzzle();
  let work = solve_puzzle(blank_puzzle);
  let solution_key = get_solution_key(&work);

  // - RSA crypto
  let mut rng = ChaCha20Rng::from_seed(solution_key);
  let bits = 2048;

  // This is all deterministic since we're using a seeded ChaCha RNG
  let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
  let public_key_pkcs8 = private_key
    .to_public_key()
    .to_pkcs1_pem(rsa::pkcs8::LineEnding::CRLF)
    .expect("failed to export the public key as pkcs8");

  // Write to zip
  let file = File::create(&path).unwrap();

  let mut zip = ZipWriter::new(file);
  let options = FileOptions::default()
    .compression_method(zip::CompressionMethod::Stored)
    .unix_permissions(0o755);

  zip.add_directory("data/", Default::default())?;

  zip.start_file("settings", options)?;
  zip.write_all(b"TODO\n")?;

  zip.start_file("public", Default::default())?;
  zip.write_all(public_key_pkcs8.as_bytes())?;

  zip.start_file("puzzle", Default::default())?;
  puzzlefile::write_puzzle(&blank_puzzle, &mut zip)?;

  zip.finish()?;
  Ok(())
}

pub fn solve(args: &cli::Solve) -> ZipResult<()> {
  let path = Path::new(&args.archive);

  if path.exists() {
    panic!("archive does not exists")
  }

  let file = File::open(&path).unwrap();
  let mut zip = ZipArchive::new(file)?;

  let puzzle_contents = zip.by_name("puzzle")?;
  let puzzle = puzzlefile::read_puzzle(&mut BufReader::new(puzzle_contents))?;

  let work = solve_puzzle(puzzle);
  let solution_key = get_solution_key(&work);

  // - RSA crypto
  let mut rng = ChaCha20Rng::from_seed(solution_key);
  let bits = 2048;

  // This is all deterministic since we're using a seeded ChaCha RNG
  let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");

  let file = File::append(&path).unwrap();
  let options = FileOptions::default()
    .compression_method(zip::CompressionMethod::Stored)
    .unix_permissions(0o755);
  zip_writer.start_file("private", options)?;
  zip_writer.write_all(private_key.to_pkcs1_pem(rsa::pkcs8::LineEnding::CRLF))?;

  Ok(())
}
