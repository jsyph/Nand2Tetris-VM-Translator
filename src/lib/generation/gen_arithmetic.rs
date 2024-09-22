use handlebars::Handlebars;

use crate::{
    error::{TranslatorError, TranslatorResult},
    parse::{line_command::LineCommand, ParsedLine},
};

#[derive(serde::Serialize)]
struct ArithmeticTemplateData {
    optimize: bool,
}

pub fn gen_arithmetic(
    handlebars: &Handlebars,
    line: ParsedLine,
    optimize: bool,
) -> TranslatorResult<String> {
    let line_command = line.command;

    let data = ArithmeticTemplateData { optimize };

    let template_name = match line_command {
        LineCommand::Add => "arithmetic/add.hbs",
        LineCommand::Neg => "arithmetic/neg.hbs",
        LineCommand::Sub => "arithmetic/sub.hbs",
        _ => return Err(TranslatorError::UnDefinedBehavior),
    };
    Ok(handlebars.render(template_name, &data)?)
}
