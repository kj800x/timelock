use crate::formats::puzzlefile;
use crate::{cli, puzzle};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use zip::result::ZipResult;
use zip::write::FileOptions;
use zip::ZipWriter;

pub fn create(args: &cli::Create) -> ZipResult<()> {
  let path = Path::new(&args.archive);

  if path.exists() {
    panic!("archive already exists, refusing to overwrite")
  }

  let file = File::create(&path).unwrap();

  let mut zip = ZipWriter::new(file);
  let options = FileOptions::default()
    .compression_method(zip::CompressionMethod::Stored)
    .unix_permissions(0o755);

  zip.add_directory("data/", Default::default())?;

  zip.start_file("settings", options)?;
  zip.write_all(b"TODO\n")?;

  zip.start_file("public", Default::default())?;
  zip.write_all(b"TODO\n")?;

  let blank_puzzle = puzzle::new_blank_puzzle();

  zip.start_file("puzzle", Default::default())?;
  puzzlefile::write_puzzle(&blank_puzzle, &mut zip)?;

  zip.start_file("private.xor", Default::default())?;
  zip.write_all(b"TODO\n")?;

  zip.finish()?;
  Ok(())

  // let work = workfile::read_work(&args.solution).expect("SolutionFile must be in valid format");
  // let key = work
  //   .last()
  //   .expect("SolutionFile must contain at least one chain")
  //   .2;

  // let mut input_file = File::open(&args.input).expect("Failed to open input file");
  // let output_file = File::create(&args.output).expect("Failed to open output file");
  // let encryptor = AesSafe256Encryptor::new(&key);
  // let mut writer = AesWriter::new(output_file, encryptor).expect("Error initializing AES");

  // std::io::copy(&mut input_file, &mut writer).expect("Failed to copy encrypted contents");
}
