use crate::core::*;
use crate::time;
use crate::workfile;
use clap::ArgMatches;

pub fn select(matches: &ArgMatches) {
    let input = matches.value_of("work").unwrap(); // Safe because defaulted in yaml
    let output = matches.value_of("solution").unwrap(); // Safe because defaulted in yaml
    let amount: Count = time::parse_time(matches.value_of("amount").unwrap());

    let work = workfile::read_work(&input).expect("Unable to read WorkFile");
    let (solution_work, out_work) = workfile::split(work, amount);

    workfile::write_work(&solution_work, false, output).expect("Unable to write SolutionFile");
    workfile::write_work(&out_work, false, input).expect("Unable to write WorkFile");
}
