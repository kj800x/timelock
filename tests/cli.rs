use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn prints_help_text() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("timelock")?;

    cmd.arg("help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage: timelock <COMMAND>"));

    Ok(())
}
