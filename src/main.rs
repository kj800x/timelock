#[macro_use]
extern crate clap;

use clap::{App, AppSettings};
mod info;
mod work;
mod workfile;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    if let Some(work_matches) = matches.subcommand_matches("work") {
        work::work(work_matches);
    }

    if let Some(info_matches) = matches.subcommand_matches("info") {
        info::info(info_matches);
    }
}
