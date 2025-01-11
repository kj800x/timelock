use std::path::PathBuf;

use clap::Parser;

mod archive;
mod cli;
mod hash;
mod hex_utils;
mod info;
mod time;
mod types;
mod work;
mod workfile;

fn main() {
    use cli::Commands::*;
    let cli = cli::Cli::parse();

    match &cli.command {
        Work(args) => {
            work::work(args);
        }
        Info(args) => {
            info::info(args);
        }
        New(args) => {
            archive::Archive::create(PathBuf::from(args.file.to_owned()));
        }
    }
}
