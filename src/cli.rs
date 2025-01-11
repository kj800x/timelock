use clap::{Args, Parser, Subcommand};

/// Generate proof of work for puzzle creation
#[derive(Args)]
pub struct Work {
    /// Sets number of threads to use
    #[clap(default_value_t = String::from("8"), short)]
    pub parallelism: String,

    /// Sets minimum target number of total iterations (0 for unlimited) (can use suffixes smhDMY)
    #[clap(default_value_t = String::from("0"), short)]
    pub target: String,

    /// Sets the max length of each individual chain (0 for unlimited) (can use suffixes smhDMY)
    #[clap(default_value_t = String::from("1m"), short)]
    pub chain_length: String,

    /// Set the WorkFile to use
    #[clap(default_value_t = String::from("timelock.work"), short)]
    pub work: String,
}

/// Determine encryption rate and predict time to solve
#[derive(Args)]
pub struct Info {
    /// Set the WorkFile to use
    #[clap(default_value_t = String::from("timelock.work"), short)]
    pub work: String,
}

/// Create a new archive
#[derive(Args)]
pub struct New {
    /// The name of the archive
    pub file: String,
}

#[derive(Subcommand)]
pub enum Commands {
    Work(Work),
    New(New),
    Info(Info),
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}
