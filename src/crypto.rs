use crate::{cli, formats::workfile};

use aesstream::{AesReader, AesWriter};
use crypto::aessafe::{AesSafe256Decryptor, AesSafe256Encryptor};

use std::fs::File;

pub fn encrypt(args: &cli::Encrypt) {
  let work = workfile::read_work(&args.solution).expect("SolutionFile must be in valid format");
  let key = work
    .last()
    .expect("SolutionFile must contain at least one chain")
    .2;

  let mut input_file = File::open(&args.input).expect("Failed to open input file");
  let output_file = File::create(&args.output).expect("Failed to open output file");
  let encryptor = AesSafe256Encryptor::new(&key);
  let mut writer = AesWriter::new(output_file, encryptor).expect("Error initializing AES");

  std::io::copy(&mut input_file, &mut writer).expect("Failed to copy encrypted contents");
}

pub fn decrypt(args: &cli::Decrypt) {
  let work = workfile::read_work(&args.solution).expect("SolutionFile must be in valid format");
  let key = work
    .last()
    .expect("SolutionFile must contain at least one chain")
    .2;

  let input_file = File::open(&args.input).expect("Failed to open input file");
  let mut output_file = File::create(&args.output).expect("Failed to open output file");
  let decryptor = AesSafe256Decryptor::new(&key);
  let mut reader = AesReader::new(input_file, decryptor).expect("Error initializing AES");

  std::io::copy(&mut reader, &mut output_file).expect("Failed to copy decrypted contents");
}
