use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli() {
    let mut cmd = Command::cargo_bin("jty-converter").unwrap();

    cmd.arg("tests/data/json/file.json")
        .arg("--toml")
        .assert()
        .success();

    std::fs::remove_file("tests/data/json/file.toml").unwrap();

    cmd.arg("tests/data/json/file.json")
        .arg("--json")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "unexpected argument 'tests/data/json/file.json' found",
        ));
}
