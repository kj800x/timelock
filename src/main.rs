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
            archive::Archive::create(args.archive.clone());
        }
        ArchiveInfo(args) => {
            archive::Archive::load(args.archive.clone()).info();
        }
        Encrypt(args) => {
            archive::Archive::load(args.archive.clone()).encrypt(
                args.file.clone(),
                args.archive_name
                    .clone()
                    .unwrap_or(args.file.to_str().map(|s| s.to_owned()).unwrap()),
            );
        }
        Decrypt(args) => {
            let target_file = args
                .file
                .clone()
                .unwrap_or(PathBuf::from(args.archive_name.to_owned()));

            if target_file.exists() {
                println!("File already exists, please provide a different path");
                return;
            }

            archive::Archive::load(args.archive.clone())
                .decrypt(args.archive_name.clone(), target_file);
        }
    }
}
