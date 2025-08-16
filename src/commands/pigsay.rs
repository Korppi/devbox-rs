use std::io::{Result, Write};

use chrono::offset;

use crate::utils::read_input;

const bubble_offset: usize = 4;

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
    width: usize,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<()>  {
    let input_text = read_input(input)?;
    let vec_text = textwrap::wrap(&input_text, width);
    
    let bubble_top = "_".repeat(width + bubble_offset * 2);
    writeln!(out, " {bubble_top}")?;
    writeln!(out, "{}{}{}", "/", " ".repeat(width + bubble_offset * 2), "\\")?;

    for text in vec_text{
        write!(out, "|{}", " ".repeat(bubble_offset))?;
        write!(out, "{text}")?;
        write!(out, "{}|", " ".repeat(bubble_offset  + width-text.len()))?;
        write!(out, "\n")?;
    }
    writeln!(out, "{}{}  {}{}", "\\","_".repeat(3), "_".repeat(width - 5 + bubble_offset * 2), "/")?;
    write!(out, "    \\|")?;

    let pig = PIG;
    let pig = pig.replace("1", &eye.to_string());
    let pig = pig.replace("2", &tail.to_string());
    writeln!(out, "{pig}")?;

    Ok(())
}
