use clap::Parser;

extern crate clap;

mod archive;
mod cli;
mod crypto;
mod formats;
mod hash;
mod info;
mod puzzle;
mod select;
mod types;
mod utils;
mod work;

fn main() {
  let cli = cli::Cli::parse();

  use cli::Commands::*;
  match &cli.command {
    Work(args) => work::work(args),
    Create(args) => archive::create(args).expect("error creating archive (TODO)"),
    Info(args) => info::info(args),
    Secure(args) => puzzle::puzzle(args),
    Solve(args) => puzzle::solve(args),
    Encrypt(args) => crypto::encrypt(args),
    Decrypt(args) => crypto::decrypt(args),
    Use(args) => select::select(args),
    _ => {
      // todo for now
    }
  }
}
