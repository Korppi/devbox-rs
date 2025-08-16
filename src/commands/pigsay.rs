use std::io::{Result, Write};

use crate::utils::read_input;



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
) -> Result<()>  {

    let input_text = read_input(input)?;
    let vec_text = textwrap::wrap(&input_text, 30);
    
    for text in vec_text{
        writeln!(out, "{text}")?;
    }

    let pig = PIG;
    let pig = pig.replace("1", &eye.to_string());
    let pig = pig.replace("2", &tail.to_string());
    writeln!(out, "{pig}")?;

    Ok(())
}
