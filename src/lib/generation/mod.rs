mod gen_arithmetic;
mod gen_logical;
mod gen_memory;
mod gen_branching;
mod gen_function;

use gen_arithmetic::gen_arithmetic;
use gen_logical::gen_logical;
use gen_memory::gen_memory;
use gen_branching::gen_branching;
use gen_function::gen_function;
use rust_embed::Embed;

use crate::{
    error::TranslatorResult,
    parse::{line_command::LineCommandType, ParsedLine},
};
use handlebars::Handlebars;

#[derive(Embed)]
#[folder = "src/lib/generation/templates/"]
#[include = "*.hbs"]
struct Templates;

pub fn generate_code(
    parsed_lines: Vec<ParsedLine>,
    file_name: &str,
    optimize: bool,
) -> TranslatorResult<Vec<String>> {
    let mut handlebars = Handlebars::new();
    let _ = handlebars.register_embed_templates::<Templates>();

    let mut res: Vec<String> = Vec::new();
    for line in parsed_lines {
        let generated_code = match line.command.command_type() {
            LineCommandType::Arithmetic => gen_arithmetic(&handlebars, line, optimize)?,
            LineCommandType::Logical => gen_logical(&handlebars, line, optimize)?,
            LineCommandType::Memory => gen_memory(&handlebars, line, file_name, optimize)?,
            LineCommandType::Branching => gen_branching(&handlebars, line, optimize)?,
            LineCommandType::Function => gen_function(&handlebars, line, optimize)?,
        };
        res.push(generated_code);
    }
    Ok(res)
}
