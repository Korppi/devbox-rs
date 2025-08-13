use std::io::{Result, Write};

const PIG: &str = r#"
         _/|________
        / 1         \
       E,            |2
        \___________/
         WW       WW
"#;

pub fn run(
    eye: char,
    tail: char,
    input: Option<String>,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<()> {
    let pig = PIG;
    let pig = pig.replace("1", &eye.to_string());
    let pig = pig.replace("2", &tail.to_string());
    writeln!(out, "{pig}")?;

    Ok(())
}
