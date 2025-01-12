use aesstream::{AesReader, AesWriter};
use crypto::aessafe::{AesSafe256Decryptor, AesSafe256Encryptor};
use radix_fmt::radix;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};
use std::{
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom, Write},
    os::unix::ffi::OsStrExt,
    path::PathBuf,
};
use uuid::Uuid;

trait AesEncryption {
    fn aes_encrypt(&self, key: &[u8; 32]) -> Vec<u8>;
    fn aes_decrypt(&self, key: &[u8; 32]) -> Vec<u8>;
}

impl AesEncryption for Vec<u8> {
    fn aes_encrypt(&self, key: &[u8; 32]) -> Vec<u8> {
        let mut encrypted = Vec::new();
        let encryptor = AesSafe256Encryptor::new(key);
        {
            let mut writer = AesWriter::new(&mut encrypted, encryptor).unwrap();
            writer.write_all(self).unwrap();
        }
        encrypted
    }

    fn aes_decrypt(&self, key: &[u8; 32]) -> Vec<u8> {
        Cursor::new(self);
        let decryptor = AesSafe256Decryptor::new(key);
        let mut reader = AesReader::new(Cursor::new(self), decryptor).unwrap();
        let mut decrypted = String::new();
        reader.read_to_string(&mut decrypted).unwrap();
        decrypted.as_bytes().to_vec()
    }
}

#[derive(Debug)]
pub struct TarListing {
    entries: Vec<(TarHeader, u64)>,
}

#[derive(Debug)]
struct TarHeader {
    name: String,
    mode: u64,
    uid: u64,
    gid: u64,
    size: u64,
    mtime: u64,
}
// name: [u8; 100], // name of file
// mode: [u8; 8],   // file mode
// uid: [u8; 8],    // user id numeric (octal)
// gid: [u8; 8],    // group id numeric (octal)
// size: [u8; 12],  // size of file in bytes (octal)
// mtime: [u8; 12], // modification time (octal)
// chksum: [u8; 8], // checksum for header
// typeflag: u8,    // file type
// linkname: [u8; 100], // name of linked file

// These disabled for now:
// magic: [u8; 6],      // "ustar\0"
// version: [u8; 2],    // "00"
// uname: [u8; 32],     // username string
// gname: [u8; 32],     // group name string
// devmajor: [u8; 8],   // device major number
// devminor: [u8; 8],   // device minor number
// prefix: [u8; 155],   // prefix for file name

trait StrExtensions {
    fn as_bytes_filled(&self, size: usize) -> Vec<u8>;
}

impl StrExtensions for str {
    fn as_bytes_filled(&self, size: usize) -> Vec<u8> {
        let mut bytes = vec![b'\0'; size];
        let s_bytes = self.as_bytes();
        bytes[..s_bytes.len()].copy_from_slice(s_bytes);
        bytes
    }
}

trait SliceExtensions {
    fn padded(&self, size: usize) -> Vec<u8>;
}

fn padding_for(buffer_size: u64, multiple_of: u64) -> u64 {
    if buffer_size % multiple_of == 0 {
        0
    } else {
        multiple_of - (buffer_size % multiple_of)
    }
}

impl SliceExtensions for [u8] {
    fn padded(&self, size: usize) -> Vec<u8> {
        let mut padded = self.to_vec();
        let padding = padding_for(self.len() as u64, size as u64);
        padded.resize(self.len() + padding as usize, b'\0');
        padded
    }
}

fn octal(n: u64, size: usize) -> Vec<u8> {
    let mut bytes = vec![b'0'; size];
    bytes[size - 1] = b'\0';

    let formatted_str = radix(n, 8).to_string();
    let formatted = formatted_str.as_bytes();
    bytes[(size - 1 - formatted.len())..size - 1].copy_from_slice(formatted);

    bytes
}

fn checksum_string(checksum: u64) -> Vec<u8> {
    let mut bytes = vec![b'0'];
    bytes.append(&mut octal(checksum, 6));
    bytes.push(b' ');
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octal() {
        let result = octal(0, 8);
        let expected = b"0000000\0";
        assert_eq!(result, expected);

        let result = octal(1, 8);
        let expected = b"0000001\0";
        assert_eq!(result, expected);

        let result = octal(9, 8);
        let expected = b"0000011\0";
        assert_eq!(result, expected);

        let result = octal(9, 12);
        let expected = b"00000000011\0";
        assert_eq!(result, expected);

        let result = octal(431, 12);
        let expected = b"00000000657\0";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_tar_header() {
        let header = TarHeader::new("test file".to_owned(), 512);

        let bytes = header.as_bytes();

        assert!(
            bytes[0..100].starts_with(b"test file\0"),
            "Name field is incorrect"
        );
        assert_eq!(bytes[100..108], *b"0000664\0", "Mode field is incorrect");
        assert_eq!(bytes[108..116], *b"0000000\0", "UID field is incorrect");
        assert_eq!(bytes[116..124], *b"0000000\0", "GID field is incorrect");
        assert_eq!(
            bytes[124..136],
            *b"00000001000\0",
            "Size field is incorrect"
        );
        assert_eq!(
            bytes[136..148],
            *b"00000000000\0",
            "Mtime field is incorrect"
        );
        assert_eq!(bytes[156], b'0', "Typeflag field is incorrect");
        assert_eq!(bytes[157..257], [0; 100], "Linkname field is incorrect");

        // Check the checksum last so that we don't get false positives where we think the checksum
        // calculation is off when it's actually the other fields that are wrong.
        // Can you tell I've been burned by this before?
        assert_eq!(
            std::str::from_utf8(&bytes[148..156]).unwrap(),
            std::str::from_utf8(b"007500\0 ").unwrap(),
            "Checksum field is incorrect"
        );
    }
}

trait VecExtensions {
    fn extend_from_with_sized_octal(&mut self, n: u64, size: usize);
    fn extend_from_with_sized_str(&mut self, s: &str, size: usize);
}

impl VecExtensions for Vec<u8> {
    fn extend_from_with_sized_octal(&mut self, n: u64, size: usize) {
        self.extend_from_slice(&octal(n, size));
    }

    fn extend_from_with_sized_str(&mut self, s: &str, size: usize) {
        self.extend_from_slice(&s.as_bytes_filled(size));
    }
}

impl TarHeader {
    fn new(name: String, size: u64) -> Self {
        Self {
            name,
            mode: 0o664,
            uid: 0,
            gid: 0,
            size,
            mtime: 0,
        }
    }

    fn from_bytes(bytes: &[u8; 512]) -> Self {
        let name = std::str::from_utf8(&bytes[0..100])
            .unwrap()
            .trim_end_matches('\0')
            .to_owned();
        let mode = u64::from_str_radix(std::str::from_utf8(&bytes[100..107]).unwrap(), 8).unwrap();
        let uid = u64::from_str_radix(std::str::from_utf8(&bytes[108..115]).unwrap(), 8).unwrap();
        let gid = u64::from_str_radix(std::str::from_utf8(&bytes[116..123]).unwrap(), 8).unwrap();
        let size = u64::from_str_radix(std::str::from_utf8(&bytes[124..135]).unwrap(), 8).unwrap();
        let mtime = u64::from_str_radix(std::str::from_utf8(&bytes[136..147]).unwrap(), 8).unwrap();

        Self {
            name,
            mode,
            uid,
            gid,
            size,
            mtime,
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_with_sized_str(&self.name, 100);
        bytes.extend_from_with_sized_octal(self.mode, 8);
        bytes.extend_from_with_sized_octal(self.uid, 8);
        bytes.extend_from_with_sized_octal(self.gid, 8);
        bytes.extend_from_with_sized_octal(self.size, 12);
        bytes.extend_from_with_sized_octal(self.mtime, 12);
        // Initially the checksum field is filled with spaces (ASCII 32)
        bytes.extend_from_slice(&[b' '; 8]);
        bytes.push(b'0'); // typeflag
        bytes.extend_from_with_sized_str("", 100); // linkname
        bytes.extend_from_slice("ustar ".as_bytes()); // Magic string
        bytes.extend_from_slice(" \0".as_bytes()); // Magic version
        bytes.extend_from_with_sized_str("", 32); // owner name str
        bytes.extend_from_with_sized_str("", 32); // group name str
        bytes.extend_from_with_sized_str("", 8); // device major
        bytes.extend_from_with_sized_str("", 8); // device minor
        bytes.extend_from_with_sized_str("", 131); // filename prefix
        bytes.extend_from_with_sized_str("", 12); // atime
        bytes.extend_from_with_sized_str("", 12); // ctime

        // Correct checksum is calculated and replaced in the header
        let correct_checksum: u64 = bytes.iter().fold(0, |acc, &byte| acc + byte as u64);
        let checksum_bytes: [u8; 8] = checksum_string(correct_checksum).try_into().unwrap();
        bytes[148..156].clone_from_slice(&checksum_bytes);

        bytes.resize(512, b'\0');

        bytes
    }
}

pub struct Archive<W: Write + Read + Seek> {
    file: W,
}

impl Archive<File> {
    pub fn create(file: PathBuf) -> Self {
        let file = File::options()
            .write(true)
            .read(true)
            .create_new(true)
            .open(file)
            .unwrap();
        let mut archive = Archive { file };
        archive.initialize();
        archive
    }

    pub fn load(file: PathBuf) -> Self {
        let file = File::options().write(true).read(true).open(file).unwrap();
        let mut archive = Archive { file };
        archive.verify().expect("Archive is not in a valid state");
        archive
    }

    pub fn info(&mut self) {
        println!("{:?}", self.listing());

        let puzzle = self.puzzle();
        println!("{}", puzzle.as_str());

        if let Some(key) = puzzle.solution() {
            println!("Solved!");
            println!("AES Key: {}", hex::encode(key));
            println!(
                "RSA Private Key: {}",
                String::from_utf8(self.read_file_contents("rsa_id.enc").aes_decrypt(&key)).unwrap()
            );

            let private_key = RsaPrivateKey::from_pkcs1_pem(
                &String::from_utf8(self.read_file_contents("rsa_id.enc").aes_decrypt(&key))
                    .unwrap(),
            )
            .unwrap();

            let uuids = self
                .listing()
                .entries
                .iter()
                .filter_map(|(header, _)| {
                    if header.name.starts_with("aes_keys/") {
                        // extract the uuid from the filename
                        let uuid = header.name.split('/').last().unwrap();
                        Some(uuid.to_owned())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>();

            for uuid in uuids {
                let encrypted_aes_key = self.read_file_contents(&format!("aes_keys/{}", uuid));
                let aes_key = private_key
                    .decrypt(Pkcs1v15Encrypt {}, &encrypted_aes_key)
                    .unwrap();

                let encrypted_filename = self.read_file_contents(&format!("filenames/{}", uuid));
                let filename = String::from_utf8(
                    encrypted_filename.aes_decrypt(&aes_key.clone().try_into().unwrap()),
                )
                .unwrap();

                println!("Filename: {}", filename);
            }
        } else {
            println!("Not solved!");
        }
    }

    pub fn encrypt(&mut self, payload: PathBuf, archive_name: String) {
        // create a new aes key
        let mut rng = ChaCha20Rng::from_entropy();
        let mut key = [0u8; 32];
        let padding = Pkcs1v15Encrypt {};
        rng.fill_bytes(&mut key);

        // generate a new uuid
        let uuid = Uuid::new_v4();

        // encrypt the payload with the aes key
        let data = std::fs::read(payload.clone()).unwrap();
        let encrypted = data.aes_encrypt(&key);

        let encrypted_filename = archive_name.as_bytes().to_vec().aes_encrypt(&key);

        // RSA encrypt the AES key with the public key
        let rsa_pub_key = RsaPublicKey::from_pkcs1_pem(
            &String::from_utf8(self.read_file_contents("rsa_id.pub")).unwrap(),
        )
        .unwrap();
        let encrypted_aes_key = rsa_pub_key.encrypt(&mut rng, padding, &key).unwrap();

        // write the encrypted payload to the archive
        self.file.seek(SeekFrom::End(0)).unwrap();
        self.write_file(&format!("contents/{}", &uuid.to_string()), &encrypted);
        self.write_file(
            &format!("filenames/{}", &uuid.to_string()),
            &encrypted_filename,
        );
        self.write_file(
            &format!("aes_keys/{}", &uuid.to_string()),
            &encrypted_aes_key,
        );
    }

    pub fn decrypt(&mut self, target_filename: String, output: PathBuf) {
        let puzzle = self.puzzle();
        let solution = if let Some(solution) = puzzle.solution() {
            solution
        } else {
            println!("Puzzle not solved, cannot decrypt");
            return;
        };

        let private_key = RsaPrivateKey::from_pkcs1_pem(
            &String::from_utf8(self.read_file_contents("rsa_id.enc").aes_decrypt(&solution))
                .unwrap(),
        )
        .unwrap();

        let listing = self.listing();
        let uuids = listing
            .entries
            .iter()
            .filter_map(|(header, _)| {
                if header.name.starts_with("aes_keys/") {
                    // extract the uuid from the filename
                    let uuid = header.name.split('/').last().unwrap();
                    Some(uuid.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        for uuid in uuids {
            let encrypted_aes_key = self.read_file_contents(&format!("aes_keys/{}", uuid));
            let aes_key = private_key
                .decrypt(Pkcs1v15Encrypt {}, &encrypted_aes_key)
                .unwrap();

            let encrypted_filename = self.read_file_contents(&format!("filenames/{}", uuid));
            let filename = String::from_utf8(
                encrypted_filename.aes_decrypt(&aes_key.clone().try_into().unwrap()),
            )
            .unwrap();

            if target_filename == filename {
                let encrypted_data = self.read_file_contents(&format!("contents/{}", uuid));
                let data = encrypted_data.aes_decrypt(&aes_key.try_into().unwrap());
                std::fs::write(output.clone(), data).unwrap();
                println!("Decrypted to: {:?}", output);
                return;
            }
        }

        println!("File not found");
    }
}

impl<W> Archive<W>
where
    W: Write + Read + Seek,
{
    fn write_file(&mut self, name: &str, data: &[u8]) {
        let header = TarHeader::new(name.to_owned(), data.len() as u64).as_bytes();
        self.file.write_all(&header).unwrap();
        self.file.write_all(&data.padded(512)).unwrap();
    }

    fn write_sparse_file(&mut self, name: &str, data: &[u8], size: u64) {
        assert!((data.len() as u64) < size);
        let header = TarHeader::new(name.to_owned(), size).as_bytes();
        self.file.write_all(&header).unwrap();

        let padded_data = &data.padded(512);
        let sparse_bytes = size - padded_data.len() as u64;

        self.file.write_all(padded_data).unwrap();
        self.file
            .seek(SeekFrom::Current(sparse_bytes as i64 - 1))
            .unwrap();
        self.file.write_all(&[0]).unwrap();
    }

    fn at_eof(&mut self) -> bool {
        let mut buffer = [0u8; 1];
        let at_eof = self.file.read(&mut buffer).unwrap() == 0;
        if at_eof {
            true
        } else {
            self.file.seek(SeekFrom::Current(-1)).unwrap();
            false
        }
    }

    fn current_position(&mut self) -> u64 {
        self.file.seek(SeekFrom::Current(0)).unwrap()
    }

    pub fn listing(&mut self) -> TarListing {
        let mut listing = TarListing {
            entries: Vec::new(),
        };
        let mut header = [0u8; 512];
        self.file.seek(SeekFrom::Start(0)).unwrap();
        loop {
            if self.at_eof() {
                break;
            }
            let header_start = self.current_position();

            self.file.read_exact(&mut header).unwrap();
            if header.iter().all(|&b| b == 0) {
                break;
            }
            let parsed_header = TarHeader::from_bytes(&header);
            let next_file_start = parsed_header.size + padding_for(parsed_header.size, 512);
            listing.entries.push((parsed_header, header_start));
            self.file
                .seek(SeekFrom::Current(next_file_start as i64))
                .unwrap();
        }
        listing
    }

    pub fn read_file_contents_zero_term(&mut self, filename: &str) -> Vec<u8> {
        let mut buffer = Vec::new();
        let listing = self.listing();
        self.file.seek(SeekFrom::Start(0)).unwrap();
        for (header, start) in listing.entries {
            if header.name == filename {
                self.file.seek(SeekFrom::Start(start + 512)).unwrap();
                // read one byte at a time until we hit a null byte
                for i in 0..header.size as usize {
                    buffer.resize(i + 1, 0);
                    self.file.read_exact(&mut buffer[i..=i]).unwrap();
                    if buffer[i] == 0 {
                        buffer.resize(i, 0);
                        break;
                    }
                }
                break;
            }
        }
        buffer
    }

    pub fn read_file_contents(&mut self, filename: &str) -> Vec<u8> {
        let mut buffer = Vec::new();
        let listing = self.listing();
        self.file.seek(SeekFrom::Start(0)).unwrap();
        for (header, start) in listing.entries {
            if header.name == filename {
                self.file.seek(SeekFrom::Start(start + 512)).unwrap();
                buffer.resize(header.size as usize, 0);
                self.file.read_exact(&mut buffer).unwrap();
                break;
            }
        }
        buffer
    }

    fn puzzle(&mut self) -> Puzzle {
        Puzzle::from_bytes(self.read_file_contents_zero_term("puzzle")).unwrap()
    }

    fn initialize(&mut self) {
        // To initialize a new archive
        // Generate an RSA Pair
        // Write the public key to the archive
        // Generate an AES key
        // Encrypt the RSA Private key with the AES key, write the AES key as a "solved" puzzle
        // Write the encrypted RSA Private key to the archive

        let mut rng = ChaCha20Rng::from_entropy();
        let priv_key = RsaPrivateKey::new(&mut rng, 2048).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);
        let mut aes_key = [0u8; 32];
        rng.fill_bytes(&mut aes_key);

        self.file.seek(SeekFrom::Start(0)).unwrap();
        self.write_file(
            "rsa_id.pub",
            pub_key
                .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
                .unwrap()
                .as_bytes(),
        );
        self.write_file(
            "rsa_id.enc",
            &priv_key
                .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
                .unwrap()
                .as_bytes()
                .to_vec()
                .aes_encrypt(&aes_key),
        );
        self.write_sparse_file(
            "puzzle",
            &Puzzle::solved_with_solution(aes_key).as_bytes(),
            1024 * 1024 * 1024,
        );
    }

    fn verify(&mut self) -> Result<(), String> {
        // TODO, implement a method to verify the archive is in a valid state
        Ok(())
    }

    // TODO, implement methods for reading and writing tar blocks, for appending or extracting certain files
    // TODO, then implement methods to set up the basic archive structure we need for this app,
    // and also purpose methods for accessing the timelock data.
}

struct Puzzle {
    entries: Vec<([u8; 32], u64)>,
}

impl Puzzle {
    fn solved_with_solution(solution: [u8; 32]) -> Self {
        let mut entries = Vec::new();
        entries.push((solution, 0));
        Self { entries }
    }

    fn solution(&self) -> Option<[u8; 32]> {
        if self.entries.len() == 1 {
            Some(self.entries[0].0)
        } else {
            None
        }
    }

    fn as_str(&self) -> String {
        let mut result = String::new();
        for (initial_value, count) in &self.entries {
            result.push_str(&format!("{}:{}\n", hex::encode(initial_value), count))
        }
        result
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.as_str().as_bytes().to_vec()
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<Self, String> {
        let mut entries = Vec::new();
        let lines = bytes.split(|&b| b == b'\n');
        for line in lines {
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split(|&b| b == b':');
            let initial_value = parts.next().unwrap();
            let count = parts.next().unwrap();
            let initial_value = hex::decode(initial_value).unwrap();
            let count = std::str::from_utf8(count).unwrap().parse().unwrap();
            entries.push((initial_value.try_into().unwrap(), count));
        }
        Ok(Self { entries })
    }
}
