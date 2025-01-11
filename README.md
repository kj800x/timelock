# Timelock

A timelock puzzle is a vector in the following format:
- Hash - 32 byte string
- Iteration Count - 64 bit unsigned integer

It serializes in the following format, with the hex encoded hash and the iteration count being joined by a colon and the entries being on separate lines:

```
eb64a545e33e49305c4bf98024e2af60d8a982355664eff7dc1fb7a9218b5ec5:4414110
06bf09461c06aaeb1b800cfa0cf1613925a4bf72efca4a0d851c3dc3aaa93f12:4414110
9fc4e8b433eae13ac852678d4d0316ee344798c6a963acab1cf55034b066e41a:4414110
abf75b5337954d0f24336c10775b83e41f0838e0d5972944f22e59d64172995c:4414110
```

In a puzzle, the first hash is given in the clear. All subsequent hashes are stored xor'd with the previous hash after it has been passed through sha256 "Iteration Count" times.

## Extending

To extend (complicate) a puzzle, you compute a new hash string (initial bytestring, iteration counts, final bytestring), update the previous first entry by xoring it's hash with the "final bytestring", and then prepend the ("initial bytestring", "iteration count") pair to the puzzle.

## Solving

To solve a link in a puzzle, you take the first hash string, hash it according to the iteration count, and then xor the result with the next hash. This removes the first link and puts the second link in the clear.

## "Solved puzzle"

A solved puzzle contains a single entry which is a hash in the clear and the iteration count 0. This means that the final result that was stored in the puzzle

```
9fc4e8b433eae13ac852678d4d0316ee344798c6a963acab1cf55034b066e41a:0
```

## Using a puzzle to encrypt a payload

To encrypt a payload using a solved timelock puzzle, extract the solution "hash" and then use it as the key for an AES 256 encryption.

# Archive Format

An archive is a TAR-based serialization of the following structure:

```
PublicKey: RSA Public Key
PrivateKey: Timelocked<RSA Private Key>
Puzzle: Timelock Puzzle for PrivateKey
Entry Keys: HashMap<UUID, RSA<AES Key>>
Entry Filenames: HashMap<UUID, AES<String>>
Entry Contents: HashMap<UUID, AES<String>>
```

With the following operations:
- solve (reduce the puzzle)
- complicate (extend the puzzle)
- encrypt (add a file to the archive)
- info (show information about the length of the puzzle yet to be solved)
- If the puzzle is fully solved, the following operations are supported.
  - list (show listing)
  - extract (decrypt and extract a single file)
  - unpack (decrypt all files and write them to disk)

And the following properties:
- The file format is easily debuggable, since it's just a tar file generally containing plain text.
- Additional files can be appended easily without having the entire archive encrypted.
- The Puzzle can be extended at any time
- The Puzzle can be partially solved at any time, and progress can be saved.
- The Puzzle can be fully solved at any time

## Tar considerations

Since we're using TAR, and we want to be able to update files in constant time inside the existing archive, we need to make sure that each file is either:
- append only
- immutable
- preallocated

It seems that most tar libraries don't support updating files in place, but the format is simple enough that we should be able to implement enough of it ourselves.

The main one that this causes issues with is the Puzzle, since it could be arbitrary sizes and needs to be able to be overwritten. The suggested solution is to just preallocate a large enough slab.

- One option may also be to make the size of the Puzzlefile configurable per archive. I think we can safely set the filesize to be whatever we want and fill the rest of the preallocated slab with NULLs. This way we can make it configurable and just read it off the header while operating on a file.

```
PublicKey: RSA Public Key (1 immutable constant space file fitting into a single tar block)
PrivateKey: Timelocked<RSA Private Key> (1 immutable constant space file fitting into a single tar block)
Puzzle: Timelock Puzzle for PrivateKey (preallocated to 1Gb. Which should fit 7.7e6 chain links, which is probably pretty big. If every chain link was sized to take 1 minute to solve, that means the longest puzzle we could fit would take 14.6 years. We should implement a check for this getting too big to fit in a 1Gb file though.)
Entry Keys: HashMap<UUID, RSA<AES Key>> (implemented as files in a known directory, append only)
Entry Filenames: HashMap<UUID, AES<String>> (implemented as files in a known directory, append only)
Entry Contents: HashMap<UUID, AES<String>> (implemented as files in a known directory, append only)
```
