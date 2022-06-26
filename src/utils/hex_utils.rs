use crate::types::*;

pub fn read_bytes(byte_str: &str) -> Result<Hash, hex::FromHexError> {
  let mut arr = [0u8; 32];
  hex::decode_to_slice(byte_str, &mut arr)?;
  Ok(arr)
}
