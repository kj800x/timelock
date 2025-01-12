use std::path::{Path, PathBuf};

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

/// List info about an archive
#[derive(Args)]
pub struct ArchiveInfo {
    /// The path to the archive
    pub archive: PathBuf,
}

/// Encrypt a file into an archive
#[derive(Args)]
pub struct Encrypt {
    /// The path to the archive
    pub archive: PathBuf,

    /// The path to the file to archive
    pub file: PathBuf,

    /// The name of the file within the archive
    /// If not provided, the file will be archived under the same path
    pub archive_name: Option<String>,
}

/// Decrypt a file from an archive
#[derive(Args)]
pub struct Decrypt {
    /// The path to the archive
    pub archive: PathBuf,

    /// The name of the file within the archive
    pub archive_name: String,

    /// The path to save the file
    /// If not provided, the file will be saved based on the archive name
    pub file: Option<PathBuf>,
}

/// Create a new archive
#[derive(Args)]
pub struct New {
    /// The path to the archive
    pub archive: PathBuf,
}

#[derive(Subcommand)]
pub enum Commands {
    Work(Work),
    New(New),
    Info(Info),
    ArchiveInfo(ArchiveInfo),
    Encrypt(Encrypt),
    Decrypt(Decrypt),
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}
