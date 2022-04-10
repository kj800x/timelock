use clap::{Args, Parser, Subcommand};

/// Generate proof of work for puzzle creation
#[derive(Args)]
pub struct Work {
    /// Sets number of threads to use
    #[clap(default_value_t = String::from("4"), short)]
    pub parallelism: String,

    /// Sets minimum target number of total iterations (0 for unlimited) (can use suffixes smhDMY)
    #[clap(default_value_t = String::from("0"), short)]
    pub target: String,

    /// Sets the max length of each individual chain (0 for unlimited) (can use suffixes smhDMY)
    #[clap(default_value_t = String::from("0"), short)]
    pub chain_length: String,

    /// Set the WorkFile to use
    #[clap(default_value_t = String::from("timelock.work"), short)]
    pub work: String,
}

/// Select amount of work to use for a puzzle
#[derive(Args)]
pub struct Use {
    /// Set the target amount of computations to use for this puzzle (0 to use entire WorkFile) (can use suffixes smhDMY)
    #[clap(default_value_t = String::from("0"))]
    pub amount: String,

    /// Set the WorkFile to use
    #[clap(default_value_t = String::from("timelock.work"), short)]
    pub work: String,

    /// Set the SolutionFile to write
    #[clap(default_value_t = String::from("timelock.soln"), short)]
    pub solution: String,
}

/// Convert a SolutionFile to a PuzzleFile
#[derive(Args)]
pub struct Puzzle {
    /// Set the SolutionFile to use
    #[clap(default_value_t = String::from("timelock.soln"), short)]
    pub solution: String,

    /// Set the PuzzleFile to write
    #[clap(default_value_t = String::from("timelock.puzl"), short)]
    pub puzzle: String,
}

/// Solve a PuzzleFile
#[derive(Args)]
pub struct Solve {
    /// Set the PuzzleFile to use
    #[clap(default_value_t = String::from("timelock.puzl"), short)]
    pub puzzle: String,

    /// Set the SolutionFile to write
    #[clap(default_value_t = String::from("timelock.soln"), short)]
    pub solution: String,
}

/// Encrypt a file
#[derive(Args)]
pub struct Encrypt {
    /// Set the file to encrypt
    #[clap()]
    pub input: String,

    /// Set where to write the encrypted file
    #[clap()]
    pub output: String,

    /// Set the SolutionFile to use
    #[clap(default_value_t = String::from("timelock.soln"), short)]
    pub solution: String,
}

/// Decrypt a file
#[derive(Args)]
pub struct Decrypt {
    /// Set the file to decrypt
    #[clap()]
    pub input: String,

    /// Set where to write the decrypted file
    #[clap()]
    pub output: String,

    /// Set the SolutionFile to use
    #[clap(default_value_t = String::from("timelock.soln"), short)]
    pub solution: String,
}

/// Determine encryption rate and predict time to solve
#[derive(Args)]
pub struct Info {
    /// Set the WorkFile to use
    #[clap(default_value_t = String::from("timelock.work"), short)]
    pub work: String,

    /// Set the SolutionFile to write
    #[clap(default_value_t = String::from("timelock.soln"), short)]
    pub solution: String,

    /// Set the PuzzleFile to use
    #[clap(default_value_t = String::from("timelock.puzl"), short)]
    pub puzzle: String,
}

#[derive(Subcommand)]
pub enum Commands {
    Work(Work),
    Use(Use),
    Puzzle(Puzzle),
    Solve(Solve),
    Encrypt(Encrypt),
    Decrypt(Decrypt),
    Info(Info),
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}
