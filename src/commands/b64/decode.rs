use ::base64::{Engine as _, engine::general_purpose};
use std::io::{self, Write};

use crate::utils::read_input;

pub fn run(
    json: bool,
    pretty: bool,
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

    // Do decoding Base64 → Vec<u8>
    let decoded_bytes = match engine.decode(&input_text) {
        Ok(bytes) => bytes,
        Err(e) => {
            writeln!(err, "Decode error: {}", e)?;
            return Ok(());
        }
    };

    //Vec<u8> → String
    let decoded_string = match String::from_utf8(decoded_bytes) {
        Ok(s) => s,
        Err(e) => {
            writeln!(err, "UTF-8 conversion error: {}", e)?;
            return Ok(());
        }
    };

    // Output
    if json {
        let value = serde_json::json!({
            "input": input_text,
            "decoded": decoded_string,
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
        writeln!(out, "{}", decoded_string)?;
    }

    Ok(())
}
