use assert_cmd::prelude::*;
use std::process::Command; // Run programs

#[test]
fn expected_day4_pt1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("aoc2023")?;

    cmd.arg("day4/input.txt")
        .arg("day4")
        .arg("pt1")
        .assert()
        .success();

    Ok(())
}

