# New Timelock Architecture

`timelock work`
Workfile is generated, saved, and used the same as before.

`timelock create`
Creates a timelock archive. This timelock archive is a ZIP archive in the following shape:

```
foo-archive.timelock:
- settings                      (this is a JSON file containing settings for the archive)
- public                        (this is the encryption key)
- puzzle                        (this is the raw puzzle)
- private.xor                   (this is the XOR of the private key and the solution)
- solution           (if solved, this is the OTP to be XOR'ed with `private.xor` to get `private`)
- private            (if solved, this is decryption key)
- data:
  - 1.dat                        (this is an AES encrypted file)
  - 1.meta                       (this is a JSON file containing metadata about the file `1.dat`. This includes the file's original filename and path relative to the archive. It is either in plaintext or encrypted based on the archive's settings)
  - 1.key                        (this is an AES key which has been encrypted using the `public` RSA key. It can be decrypted using `private`)
  - 2.dat
  - 2.meta
  - 2.key
```

ZIP has been chosen to support random access.
Some settings are immutable or only modifiable one-way under certain conditions (`metadata-encrypted`). Other settings may be changed freely (`delete-on-encrypt`).

`timelock use`
Extend the puzzle (by prepending work entries)

`timelock settings`
Print or change the archive's settings.

`timelock solve`
Solves a timelock archive by calculating the solution to the chained-hashes puzzle and saving it into the archive

`timelock secure`
Deletes the solution file and private key from the archive

`timelock list`
If metadata is not encrypted or the puzzle has been solved, print out all the files in the archive. Otherwise, error out.

`timelock encrypt`
Encrypts a given file (or set of files by glob) and save to the archive. By default this does not delete the original files, but that can be customized by settings.

`timelock decrypt`
Decrypt a given file (or set of files by regex) from the archive. Write them to the local filesystem (relative to CWD and creating directories as needed)
