use crate::{cli, workfile};

use aesstream::{AesReader, AesWriter};
use crypto::aessafe::{AesSafe256Decryptor, AesSafe256Encryptor};
use rand::{RngCore, SeedableRng};
use rand_chacha::{ChaCha20Rng, ChaChaRng};
use rand_core::OsRng;
use rsa::{
    pkcs1::{DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
    pkcs8::{DecodePrivateKey, EncodePublicKey},
    PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey,
};

use std::{
    fs::File,
    io::{Read, Write},
};

pub fn encrypt(args: &cli::RsaEncrypt) {
    // let mut input_file = File::open(&args.input).expect("Failed to open input file");
    // let output_file = File::create(&args.output).expect("Failed to open output file");
    // let encryptor = AesSafe256Encryptor::new(&key);
    // let mut writer = AesWriter::new(output_file, encryptor).expect("Error initializing AES");

    // std::io::copy(&mut input_file, &mut writer).expect("Failed to copy encrypted contents");
}

pub fn decrypt(args: &cli::RsaDecrypt) {
    let mut private_key_file =
        File::open(&args.private_key).expect("failed to open the private key file for writing");
    let mut pem = String::new();
    private_key_file
        .read_to_string(&mut pem)
        .expect("failed to read the private key file");
    let private_key =
        RsaPrivateKey::from_pkcs8_pem(&pem).expect("failed to parse pkcs8 file to a private key");

    let mut seed = [0u8; 32];
    OsRng.fill_bytes(&mut seed);
    let mut rng = ChaCha20Rng::from_seed(seed);
    let padding = PaddingScheme::new_pkcs1v15_encrypt();

    private_key.encrypt(&mut rng, padding, msg)

    // let work = workfile::read_work(&args.solution).expect("SolutionFile must be in valid format");
    // let key = work
    //     .last()
    //     .expect("SolutionFile must contain at least one chain")
    //     .2;

    // let input_file = File::open(&args.input).expect("Failed to open input file");
    // let mut output_file = File::create(&args.output).expect("Failed to open output file");
    // let decryptor = AesSafe256Decryptor::new(&key);
    // let mut reader = AesReader::new(input_file, decryptor).expect("Error initializing AES");

    // std::io::copy(&mut reader, &mut output_file).expect("Failed to copy decrypted contents");
}

pub fn keys(args: &cli::RsaKeys) {
    let work = workfile::read_work(&args.solution).expect("SolutionFile must be in valid format");
    let key = work
        .last()
        .expect("SolutionFile must contain at least one chain")
        .2;

    let mut rng = ChaChaRng::from_seed(key);
    let bits = 2048;

    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let private_key_pkcs8 = private_key
        .to_pkcs1_pem(rsa::pkcs8::LineEnding::CRLF)
        .expect("failed to export the private key as pkcs8");
    let public_key_pkcs8 = private_key
        .to_public_key()
        .to_pkcs1_pem(rsa::pkcs8::LineEnding::CRLF)
        .expect("failed to export the public key as pkcs8");

    let mut private_key_file =
        File::create(&args.private_key).expect("failed to open the private key file for writing");
    private_key_file
        .write_all(private_key_pkcs8.as_bytes())
        .expect("failed to write the private key file");

    let mut public_key_file =
        File::create(&args.public_key).expect("failed to open the public key file for writing");
    public_key_file
        .write_all(public_key_pkcs8.as_bytes())
        .expect("failed to write the public key file");
}
