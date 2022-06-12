use clap::Parser;

extern crate clap;

mod cli;
mod core;
mod crypto;
mod hash;
mod hex_utils;
mod info;
mod puzzle;
mod puzzlefile;
mod rsa;
mod select;
mod time;
mod work;
mod workfile;

fn main() {
    let cli = cli::Cli::parse();

    use cli::*;
    match &cli.command {
        Commands::Work(args) => {
            work::work(args);
        }
        Commands::Info(args) => {
            info::info(args);
        }
        Commands::Puzzle(args) => {
            puzzle::puzzle(args);
        }
        Commands::Solve(args) => {
            puzzle::solve(args);
        }
        Commands::Encrypt(args) => {
            crypto::encrypt(args);
        }
        Commands::Decrypt(args) => {
            crypto::decrypt(args);
        }
        Commands::Use(args) => {
            select::select(args);
        }
        Commands::Rsa(args) => match &args.command {
            RsaCommands::Encrypt(args) => {
                rsa::encrypt(args);
            }
            RsaCommands::Decrypt(args) => {
                rsa::decrypt(args);
            }
            RsaCommands::Keys(args) => {
                rsa::keys(args);
            }
        },
    }
}
