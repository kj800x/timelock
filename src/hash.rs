use crate::core::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rand::rngs::OsRng;
use rand_core::RngCore;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

pub fn hash(iv: Hash, stopped: &AtomicBool) -> Chain {
  let mut sha = Sha256::new();
  let mut bytes = iv;
  let mut i = 0;
  loop {
    if stopped.load(Ordering::Relaxed) {
      break;
    }
    sha.input(&bytes);
    sha.result(&mut bytes);
    sha.reset();
    i = i + 1;
  }
  return (iv, i, bytes);
}

pub fn hash_count(iv: Hash, count: u64) -> Hash {
  let mut sha = Sha256::new();
  let mut bytes = iv;
  for _ in 0..count {
    sha.input(&bytes);
    sha.result(&mut bytes);
    sha.reset();
  }
  return bytes;
}

pub fn time_hash(count: i32) -> std::time::Duration {
  let mut sha = Sha256::new();
  let mut bytes = [0u8; 32];
  OsRng.fill_bytes(&mut bytes);
  let start = Instant::now();

  for _ in 0..count {
    sha.input(&bytes);
    sha.result(&mut bytes);
    sha.reset();
  }

  let end = Instant::now();
  end.duration_since(start)
}
