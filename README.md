# Timelock
Rust implementation of [chained hashes timelock encryption](https://www.gwern.net/Self-decrypting-files#hashing) using SHA256.

## Install

```
git clone https://github.com/kj800x/timelock.git
cd timelock
cargo build --release
sudo cp ./target/release/timelock /usr/bin/timelock
```

## Encryption Usage
### Generate Work
First you need to invest work to create a workfile. The idea is that you can do this step in parallel,
but you must invest as many computations as you want to require for the decryption. Running `timelock work`
will generate a `timelock.work` file.
```
$ timelock work
Work is being generated... Press CTRL+C to stop and save progress.
^C
Chain 0: 7251387 iterations
	Initial Seed: 4364e02bd9a02bc454618bf3236a61efca37ecc4f8e020b6c43fe8b06e3f3d26
	Result Hash : 9ee1c5cf736f32b52070ee3243d130dc740c7e925e913d96fed7e63edd5ce507
Chain 1: 7258992 iterations
	Initial Seed: 5876611691e037255e7a6962ae1200c45aaf3ad5f027caf300f120f9c8849466
	Result Hash : 6f13dd8c674432977e8db34bff070dd5f898325d2cf0015e8db95649da6c0783
Chain 2: 7245372 iterations
	Initial Seed: ed020414ae18f61d48f5f177d6ec527d54f008643aef883c5e665561386ec292
	Result Hash : d8d739b4af8a5def80e96366e997745222b69e8b0458d57c73ff5b88ee9583f2
Chain 3: 7253198 iterations
	Initial Seed: 9da6fbef4bad455a7a1836e355b9cb7187e60a357ce3ca93bae3bff65e26dbc6
	Result Hash : 64dca90ab10a39ffef9e826140fd7522ddfefc2831e5914149ec4b31f0926bfe
```
* You can re-run `timelock work` to continue to append additional computation power to your workfile.
* You can also simply concatenate multiple workfiles to create a workfile with the combined computation power.
* By default the workfile is named `timelock.work` and the puzzlefile is named `timelock.puzzle` but you
  can customize those filenames using the `-w` and `-z` flags.

### Encrypt a file
Using the workfile, you can use AES256 to encrypt a file.
```
$ timelock encrypt plaintext ciphertext
```

### Build a puzzle
Using the workfile, you can generate a corresponding puzzlefile which you can make public. This time complexity of this 
conversion is linear with respect to the number of chains, which means it should be near instant. (You normally have a small
number of chains, where each chain has a high iteration count)
```
$ timelock puzzle
```

### Sharing a crypto puzzle
To share a puzzle, you need to share the ciphertext along with the puzzle file. Anyone who is interested in investing
enough computational power will be able to decrypt your message, but you can be sure that they have invested the hashing
power required. (Disclaimer: I don't see a way to solve this faster than doing the required hashes to solve the key,
but someone smarter than me may be able to break this crypto)

## Decryption Usage
### Estimate difficulty
You can run `timelock info` to get statistics about your computer's computatial power and the estimated time to solve for a
workfile or puzzlefile. Keep in mind that this is an estimated time to solve on your machine, and the time to solve will vary
based on available computation power. An ASIC can compute these hashes [way faster than your laptop
can](https://en.bitcoin.it/wiki/Mining_hardware_comparison).

```
$ timelock info
Calculating approximate hash rate...

This computer can calculate about 3491563 hashes per second

The puzzlefile contains the work of 26385959 hashes
It would take about 7 seconds to solve the puzzlefile
```

### Solve a puzzle
You can solve a puzzle and reconstruct the corresponding workfile with `timelock solve`. This process can potentially
take a long time, and progress is not saved (yet).

```
$ timelock solve
Beginning to solve chain 0 which is 6594290 computations long
Beginning to solve chain 1 which is 6594612 computations long
Beginning to solve chain 2 which is 6607702 computations long
Beginning to solve chain 3 which is 6589355 computations long
Puzzle solved!
```

### Decrypt a file
Using the reconstructed workfile, you can use AES256 to decrypt a file.
```
$ timelock decrypt ciphertext plaintext
```

# License: [MIT](./LICENSE)
