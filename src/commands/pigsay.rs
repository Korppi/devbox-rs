use std::io::{Result, Write};

use crate::utils::read_input;

const BUBBLE_OFFSET: usize = 4;

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
    mut width: usize,
    out: &mut dyn Write,
    _: &mut dyn Write,
) -> Result<()> {
    let input_text = read_input(input)?;

    if input_text.chars().count() < width {
        width = input_text.chars().count() as usize;
        width = width.clamp(5, 100);
    }

    let vec_text = textwrap::wrap(&input_text, width);

    let bubble_top = "_".repeat(width + BUBBLE_OFFSET * 2);
    writeln!(out, " {bubble_top}")?;
    writeln!(out, "/{}\\", " ".repeat(width + BUBBLE_OFFSET * 2))?;

    for text in vec_text {
        write!(out, "|{}", " ".repeat(BUBBLE_OFFSET))?;
        write!(out, "{text}")?;
        write!(out, "{}|", " ".repeat(BUBBLE_OFFSET + width - text.chars().count()))?;
        writeln!(out)?;
    }
    writeln!(
        out,
        "\\{}  {}/",
        "_".repeat(3),
        "_".repeat(width - 5 + BUBBLE_OFFSET * 2)
    )?;
    write!(out, "    \\|")?;

    let pig = PIG;
    let pig = pig.replace("1", &eye.to_string());
    let pig = pig.replace("2", &tail.to_string());
    writeln!(out, "{pig}")?;

    Ok(())
}
