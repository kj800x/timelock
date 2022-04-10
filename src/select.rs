use crate::cli;
use crate::core::*;
use crate::time;
use crate::workfile;

pub fn select(args: &cli::Use) {
    let amount: Count = time::parse_time(&args.amount);

    let work = workfile::read_work(&&args.work).expect("Unable to read WorkFile");
    let (solution_work, out_work) = workfile::split(work, amount);

    workfile::write_work(&solution_work, false, &args.solution)
        .expect("Unable to write SolutionFile");
    workfile::write_work(&out_work, false, &args.work).expect("Unable to write WorkFile");
}
