use std::io::{self, Read, Write};
use ::base64::{engine::general_purpose, Engine as _};

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
    let input_text = match input {
        Some(text) => text,
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer.trim_end().to_string()
        }
    };

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
            Ok(j) => writeln!(out, "{}", j)?,
            Err(e) => writeln!(err, "JSON error: {}", e)?,
        }
    } else {
        writeln!(out, "{}", encoded)?;
    }

    Ok(())
}