use handlebars::Handlebars;
use uuid::Uuid;

use crate::{
    error::{TranslatorError, TranslatorResult},
    parse::{line_command::LineCommand, ParsedLine},
};

#[derive(serde::Serialize)]
struct LogicalTemplateData {
    random_addr: String,
    optimize: bool,
}

pub fn gen_logical(
    handlebars: &Handlebars,
    line: ParsedLine,
    optimize: bool,
) -> TranslatorResult<String> {
    let line_command = line.command;
    let uuid = Uuid::new_v4().simple().to_string().to_uppercase();

    let random_addr = format!("{}_{}", line_command.to_string().to_uppercase(), uuid);
    let data = LogicalTemplateData {
        random_addr,
        optimize,
    };

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
