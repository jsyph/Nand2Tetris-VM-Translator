mod gen_arithmetic;
mod gen_logical;
mod gen_memory;

use gen_memory::gen_memory;
use gen_logical::gen_logical;
use gen_arithmetic::gen_arithmetic;
use rust_embed::Embed;

use crate::{error::TranslatorResult, parse::{
    line_command::LineCommandType,
    ParsedLine,
}};
use handlebars::Handlebars;

#[derive(Embed)]
#[folder = "src/lib/generation/templates/"]
#[include = "*.hbs"]
struct Templates;

pub fn generate_code(parsed_lines: Vec<ParsedLine>, file_name: &str) -> TranslatorResult<Vec<String>> {
    let mut handlebars = Handlebars::new();
    let _ = handlebars.register_embed_templates::<Templates>();

    let mut res: Vec<String> = Vec::new();
    for line in parsed_lines {
        let generated_code = match line.command.command_type() {
            LineCommandType::Arithmetic => gen_arithmetic(&handlebars, line)?,
            LineCommandType::Logical => gen_logical(&handlebars, line)?,
            LineCommandType::Memory => gen_memory(&handlebars, line, file_name)?,
        };
        res.push(generated_code);
    }
    Ok(res)
}
