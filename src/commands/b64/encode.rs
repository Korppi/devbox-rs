use ::base64::{Engine as _, engine::general_purpose};
use std::io::{self, Write};

use crate::utils::read_input;

pub fn run(
    json: bool,
    pretty: bool,
    no_pad: bool,
    urlsafe: bool,
    input: Option<String>,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> io::Result<()> {
    // Get input string: from argument or stdin
    let input_text = read_input(input)?;

    // Select engine
    let engine = if urlsafe {
        general_purpose::URL_SAFE_NO_PAD
    } else {
        general_purpose::STANDARD
    };

    // Do encoding
    let mut encoded = engine.encode(&input_text);

    // Remove padding if requested
    if no_pad {
        encoded = encoded.trim_end_matches('=').to_string();
    }

    // Output
    if json {
        let value = serde_json::json!({
            "input": input_text,
            "encoded": encoded,
        });
        let encoded_json = if pretty {
            serde_json::to_string_pretty(&value)
        } else {
            serde_json::to_string(&value)
        };

        match encoded_json {
            Ok(j) => writeln!(out, "{j}")?,
            Err(e) => writeln!(err, "JSON error: {e}")?,
        }
    } else {
        writeln!(out, "{encoded}")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_without_flags() {
        let mut out = Vec::new();
        let mut err = Vec::new();

        run(
            false,
            false,
            false,
            false,
            Some("Hello".to_string()),
            &mut out,
            &mut err,
        )
        .unwrap();
        let stdout = String::from_utf8_lossy(&out);

        assert!(
            stdout.contains("SGVsbG8="),
            "Expected SGVsbG8=, got {:?}",
            stdout
        );
    }

    #[test]
    fn test_run_json_pretty() {
        let mut out = Vec::new();
        let mut err = Vec::new();

        run(
            true,
            true,
            false,
            false,
            Some("A".to_string()),
            &mut out,
            &mut err,
        )
        .unwrap();

        let stdout = String::from_utf8_lossy(&out);
        // This means JSON is split on two rows
        assert!(stdout.contains(r#""input": "A""#));
        assert!(stdout.contains(r#""encoded": "QQ==""#));
    }
}
