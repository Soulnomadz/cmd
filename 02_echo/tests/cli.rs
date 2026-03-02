use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], exp_file: &str) -> TestResult {
    let exp = std::fs::read_to_string(exp_file)?;

    Command::cargo_bin("echor")?
	.args(args)
	.assert()
	.success()
	.stdout(exp);

    Ok(())
}

#[test]
fn die_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
	.failure()
	.stderr(predicate::str::contains("Usage"));

    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello")
	.assert()
	.success()
	.stdout("hello\n");
	
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["Hello", "there", "-n"], "tests/expected/hello2.n.txt")
}

