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
mod select;
mod time;
mod work;
mod workfile;

fn main() {
  let cli = cli::Cli::parse();

  use cli::Commands::*;
  match &cli.command {
    Work(args) => {
      work::work(args);
    }
    Info(args) => {
      info::info(args);
    }
    Puzzle(args) => {
      puzzle::puzzle(args);
    }
    Solve(args) => {
      puzzle::solve(args);
    }
    Encrypt(args) => {
      crypto::encrypt(args);
    }
    Decrypt(args) => {
      crypto::decrypt(args);
    }
    Use(args) => {
      select::select(args);
    }
  }
}
