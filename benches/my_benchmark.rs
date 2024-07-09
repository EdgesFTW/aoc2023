use assert_cmd::prelude::*;
use criterion::{criterion_group, criterion_main, Criterion};
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

fn criter_2_2(c: &mut Criterion) {
    c.bench_function("day2 pt2", |b| b.iter(|| bench_day2_pt2().unwrap()));
    c.bench_function("day2 pt2-short", |b| {
        b.iter(|| bench_day2_pt2_short().unwrap())
    });
}

fn bench_day2_pt2_short() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("aoc2023")?;

    cmd.arg("day2/input.txt")
        .arg("day2")
        .arg("pt2")
        .assert()
        .success();

    Ok(())
}

fn bench_day2_pt2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("aoc2023")?;

    cmd.arg("day2/med-input.txt")
        .arg("day2")
        .arg("pt2")
        .assert()
        .success();

    Ok(())
}

criterion_group!(benches, criter_2_2);
criterion_main!(benches);
