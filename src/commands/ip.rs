use local_ip_address::local_ip;
use reqwest::blocking::get;
use std::io::{Result, Write};

pub fn run(
    show_public: bool,
    show_local: bool,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<()> {
    let show_both = !show_public && !show_local;

    if show_public || show_both {
        match get("https://api.ipify.org") {
            Ok(res) => match res.text() {
                Ok(ip) => writeln!(out, "Public IP: {}", ip)?,
                Err(_) => writeln!(err, "Error: Failed to read public IP response.")?,
            },
            Err(_) => writeln!(err, "Error: Failed to fetch public IP.")?,
        }
    }

    if show_local || show_both {
        match local_ip() {
            Ok(ip) => writeln!(out, "Local IP: {}", ip)?,
            Err(_) => writeln!(err, "Error: Failed to get local IP.")?,
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_local_only() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let _ = run(false, true, &mut out, &mut err);
        let stdout = String::from_utf8_lossy(&out);
        let stderr = String::from_utf8_lossy(&err);

        // Allow both success and fail
        assert!(
            stdout.contains("Local IP:") || stderr.contains("Failed to get local IP"),
            "Expected local IP or error message"
        );
    }

    #[test]
    fn test_run_public_only() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let _ = run(true, false, &mut out, &mut err);
        let stdout = String::from_utf8_lossy(&out);
        let stderr = String::from_utf8_lossy(&err);

        // Allow both success and fail
        assert!(
            stdout.contains("Public IP:")
                || stderr.contains("Failed to fetch")
                || stderr.contains("Failed to read"),
            "Expected public IP or error message"
        );
    }

    #[test]
    fn test_run_both() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let _ = run(false, false, &mut out, &mut err);
        let stdout = String::from_utf8_lossy(&out);
        let stderr = String::from_utf8_lossy(&err);

        // Allow both success and fail
        assert!(
            stdout.contains("Local IP:") || stderr.contains("Failed to get local IP"),
            "Expected local IP or error message"
        );

        // Allow both success and fail
        assert!(
            stdout.contains("Public IP:")
                || stderr.contains("Failed to fetch")
                || stderr.contains("Failed to read"),
            "Expected public IP or error message"
        );
    }
}
