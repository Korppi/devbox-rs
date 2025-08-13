use std::io::{Result, Write};

const PIG: &str = r#"
         _/|________
        / o         \
       E,            |S
        \___________/
         WW       WW
"#;

pub fn run(
    input: Option<String>,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<()> {
    writeln!(out, "{PIG}")?;
        
    Ok(())
}