use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_rspec_report() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("split-test")?;

    cmd.current_dir("tests/fixtures/rspec")
        .arg("--junit-xml-report-dir")
        .arg("report")
        .arg("--node-index")
        .arg("0")
        .arg("--node-total")
        .arg("2")
        .arg("--tests-glob")
        .arg("*_spec.rb");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("a_spec.rb"))
        .stdout(predicate::str::contains("b_spec.rb").not())
        .stdout(predicate::str::contains("c_spec.rb"));

    cmd = Command::cargo_bin("split-test")?;

    cmd.current_dir("tests/fixtures/rspec")
        .arg("--junit-xml-report-dir")
        .arg("report")
        .arg("--node-index")
        .arg("1")
        .arg("--node-total")
        .arg("2")
        .arg("--tests-glob")
        .arg("*_spec.rb");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("a_spec.rb").not())
        .stdout(predicate::str::contains("b_spec.rb"))
        .stdout(predicate::str::contains("c_spec.rb").not());

    Ok(())
}

#[test]
fn test_minitest_report() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("split-test")?;

    cmd.current_dir("tests/fixtures/minitest")
        .arg("--junit-xml-report-dir")
        .arg("report")
        .arg("--node-index")
        .arg("0")
        .arg("--node-total")
        .arg("2")
        .arg("--tests-glob")
        .arg("*_test.rb");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("a_test.rb"))
        .stdout(predicate::str::contains("b_test.rb").not())
        .stdout(predicate::str::contains("c_test.rb"));

    cmd = Command::cargo_bin("split-test")?;

    cmd.current_dir("tests/fixtures/minitest")
        .arg("--junit-xml-report-dir")
        .arg("report")
        .arg("--node-index")
        .arg("1")
        .arg("--node-total")
        .arg("2")
        .arg("--tests-glob")
        .arg("*_test.rb");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("a_test.rb").not())
        .stdout(predicate::str::contains("b_test.rb"))
        .stdout(predicate::str::contains("c_test.rb").not());

    Ok(())
}
