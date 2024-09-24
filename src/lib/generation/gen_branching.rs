use handlebars::Handlebars;

use crate::{
    error::{TranslatorError, TranslatorResult},
    parse::{line_command::LineCommand, ParsedLine},
};

#[derive(serde::Serialize)]
struct BranchingTemplateData<'a> {
    label: &'a str,
    optimize: bool,
}

pub fn gen_branching(
    handlebars: &Handlebars,
    line: ParsedLine,
    optimize: bool,
) -> TranslatorResult<String> {


    let line_command = line.command;
    let label = line.label.unwrap();
    let data = BranchingTemplateData {
        optimize,
        label: &label,
    };

    let template_name = match line_command {
        LineCommand::GoTo => "branching/goto.hbs",
        LineCommand::IfGoTo => "branching/if-goto.hbs",
        LineCommand::Label => "branching/label.hbs",
        _ => return Err(TranslatorError::UnDefinedBehavior),
    };
    Ok(handlebars.render(template_name, &data)?)
}
