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
        .stdout(predicate::str::contains("c_spec.rb"))
        .stderr(predicate::str::is_empty());

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
        .stdout(predicate::str::contains("c_spec.rb").not())
        .stderr(predicate::str::is_empty());

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
        .stdout(predicate::str::contains("c_test.rb"))
        .stderr(predicate::str::is_empty());

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
        .stdout(predicate::str::contains("c_test.rb").not())
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn test_cypress_report() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("split-test")?;

    cmd.current_dir("tests/fixtures/cypress")
        .arg("--junit-xml-report-dir")
        .arg("report")
        .arg("--node-index")
        .arg("0")
        .arg("--node-total")
        .arg("2")
        .arg("--tests-glob")
        .arg("**/*_spec.js");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("a_spec.js"))
        .stdout(predicate::str::contains("b_spec.js").not())
        .stdout(predicate::str::contains("c_spec.js"));

    cmd = Command::cargo_bin("split-test")?;

    cmd.current_dir("tests/fixtures/cypress")
        .arg("--junit-xml-report-dir")
        .arg("report")
        .arg("--node-index")
        .arg("1")
        .arg("--node-total")
        .arg("2")
        .arg("--tests-glob")
        .arg("**/*_spec.js");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("a_spec.js").not())
        .stdout(predicate::str::contains("b_spec.js"))
        .stdout(predicate::str::contains("c_spec.js").not());

    Ok(())
}

#[test]
fn test_multiple_tests_glob_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("split-test")?;

    cmd.current_dir("tests/fixtures/minitest")
        .arg("--junit-xml-report-dir")
        .arg("report")
        .arg("--node-index")
        .arg("0")
        .arg("--node-total")
        .arg("1")
        .arg("--tests-glob")
        .arg("a_test.rb")
        .arg("--tests-glob")
        .arg("b_test.rb");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("a_test.rb"))
        .stdout(predicate::str::contains("b_test.rb"))
        .stdout(predicate::str::contains("c_test.rb").not())
        .stderr(predicate::str::is_empty());

    cmd = Command::cargo_bin("split-test")?;

    cmd.current_dir("tests/fixtures/minitest")
        .arg("--junit-xml-report-dir")
        .arg("report")
        .arg("--node-index")
        .arg("0")
        .arg("--node-total")
        .arg("1")
        .arg("--tests-glob")
        .arg("b_test.rb")
        .arg("--tests-glob")
        .arg("c_test.rb");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("a_test.rb").not())
        .stdout(predicate::str::contains("b_test.rb"))
        .stdout(predicate::str::contains("c_test.rb"))
        .stderr(predicate::str::is_empty());

    Ok(())
}
