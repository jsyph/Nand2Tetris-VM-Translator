use handlebars::Handlebars;
use random_string::generate;

use crate::{
    error::{TranslatorResult, TranslatorError},
    parse::{line_command::LineCommand, ParsedLine},
};

#[derive(serde::Serialize)]
struct LogicalTemplateData {
    random_addr: String,
    optimize: bool,
}

const CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ123456789";
const RAND_STRING_SIZE: usize = 10;
pub fn gen_logical(handlebars: &Handlebars, line: ParsedLine, optimize: bool) -> TranslatorResult<String> {
    log::debug!("Generating code for {:?}", &line);
    
    let line_command = line.command;


    let random_addr = format!(
        "{}_{}",
        line_command.to_string().to_uppercase(),
        generate(RAND_STRING_SIZE, CHARSET)
    );
    let data = LogicalTemplateData { random_addr, optimize };

    let template_name = match line_command {
        LineCommand::Equal => "logical/eq.hbs",
        LineCommand::LessThan => "logical/lt.hbs",
        LineCommand::GreaterThan => "logical/gt.hbs",
        LineCommand::And => "logical/and.hbs",
        LineCommand::Or => "logical/or.hbs",
        LineCommand::Not => "logical/not.hbs",
        _ => return Err(TranslatorError::UnDefinedBehavior),
    };
    Ok(handlebars.render(template_name, &data)?)
}
