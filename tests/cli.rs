use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use std::process::Command; // Run programs // Add methods on commands

#[test]
fn test_happy_path() {
    let mut cmd = Command::cargo_bin("mcdmrs").expect("executable not found");

    cmd.arg("--alternatives")
        .arg("./examples/data/alternatives.csv")
        .arg("--criteria")
        .arg("./examples/data/criteria.csv");

    cmd.assert().success();
}

#[test]
fn test_file_doesnt_exist_buffer() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mcdmrs")?;

    cmd.arg("--alternatives")
        .arg("test/file/doesnt/exist")
        .arg("--criteria")
        .arg("test/file/doesnt/exist");

    cmd.assert().failure();

    Ok(())
}
