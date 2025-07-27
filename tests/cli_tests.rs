#[cfg(test)]
mod cli_tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn encode_pretty_json_outputs_formatted() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("devbox")?;
        cmd.args(&[
                "base64", "encode",
                "--json", "--pretty",
                "Hello"
            ])
            .assert()
            .success()
            // Own row shows "input": "Hello"
            .stdout(predicate::str::contains(r#""input": "Hello""#))
            // and in second row shows "encoded": "SGVsbG8="
            .stdout(predicate::str::contains(r#""encoded": "SGVsbG8=""#));
        Ok(())
    }

    #[test]
    fn encode_pretty_without_json() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("devbox")?;
        cmd.args(&["base64", "encode", "--pretty", "Hello"])
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "error: the following required arguments were not provided:",
            ))
            .stderr(predicate::str::contains("--json"));  
        Ok(())
    }

    #[test]
    fn encode_reads_from_stdin() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("devbox")?;
        cmd.args(&["base64", "encode"])
        .write_stdin("Hello\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("SGVsbG8="));
        Ok(())
    }
}


