use std::io::{self, Read};

/// Reads either the provided `input` string, or if `None`, reads all of stdin until EOF.
/// Trims any trailing newlines from the buffered input.
pub fn read_input(input: Option<String>) -> io::Result<String> {
    match input {
        Some(text) => Ok(text),
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer.trim_end().to_string())
        }
    }
}
