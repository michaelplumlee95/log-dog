use assert_cmd::cargo;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn help_works() {
    cargo::cargo_bin_cmd!("log-dog")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("log-dog"));
}

#[test]
fn gen_and_incidents_work() {
    let dir = tempdir().unwrap();
    let out = dir.path().join("generated.jsonl");

    cargo::cargo_bin_cmd!("log-dog")
        .args([
            "gen",
            "--out",
            out.to_str().unwrap(),
            "--seed",
            "42",
            "--incident-count",
            "3",
        ])
        .assert()
        .success();

    cargo::cargo_bin_cmd!("log-dog")
        .args(["incidents", "--input", out.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Incident"));
}
