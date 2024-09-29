#[cfg(test)]

mod tests {
    use assert_cmd::Command;

    fn crate_command() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
    }

    #[test]
    fn test_crate_name() {
        let crate_name = env!("CARGO_PKG_NAME");
        assert_eq!(crate_name, "nbd");
    }

    #[test]
    fn test_cli_help() {
        let mut cmd = crate_command();
        cmd.arg("--help");

        let stdout = format!("Usage: {} <COMMAND>", env!("CARGO_PKG_NAME"));

        cmd.assert()
            .success()
            .stdout(predicates::str::contains(stdout));
    }

    #[test]
    fn test_cli_works() {
        let mut cmd = crate_command();
        cmd.arg("create")
            .arg("--first-name")
            .arg("First")
            .arg("--last-name")
            .arg("Last")
            .arg("--email")
            .arg("test@test.com")
            .arg("--phone-number")
            .arg("123-321-1233");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("Successfully saved contact"));
    }

    #[test]
    fn test_nbd_invalid_email() {
        let mut cmd = crate_command();
        cmd.arg("create")
            .arg("--first-name")
            .arg("First")
            .arg("--last-name")
            .arg("Last")
            .arg("--email")
            .arg("test@.com")
            .arg("--phone-number")
            .arg("123-321-1233");

        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("InvalidEmail"));
    }

    #[test]
    fn test_cli_invalid_phone() {
        let mut cmd = crate_command();
        cmd.arg("create")
            .arg("--first-name")
            .arg("First")
            .arg("--last-name")
            .arg("Last")
            .arg("--email")
            .arg("test@com.com")
            .arg("--phone-number")
            .arg("123-321-123");

        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("InvalidPhone"));
    }

    #[test]
    fn test_cli_invalid_args() {
        let mut cmd = crate_command();
        cmd.arg("First").arg("Last").arg("32321123");

        let stderr = format!("Usage: {} <COMMAND>", env!("CARGO_PKG_NAME"));

        cmd.assert()
            .failure()
            .stderr(predicates::str::contains(stderr));
    }
}
