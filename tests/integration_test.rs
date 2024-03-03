use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli() {
    let mut cmd = Command::cargo_bin("jty-converter").unwrap();

    cmd.arg("data/json/file.json")
        .arg("--toml")
        .assert()
        .success();

    std::fs::remove_file("data/json/file.toml").unwrap();

    cmd.arg("data/json/file.json")
        .arg("--json")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "unexpected argument 'data/json/file.json' found",
        ));
}
