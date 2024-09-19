use handlebars::Handlebars;
use random_string::generate;

use crate::parse::{line_command::LineCommand, ParsedLine};

#[derive(serde::Serialize)]
struct LogicalTemplateData {
    random_addr: String,
}

const CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ123456789";
const RAND_STRING_SIZE: usize = 10;
pub fn gen_logical(handlebars: &Handlebars, line: ParsedLine) -> String {
    let line_command = line.command;

    let random_addr = format!(
        "{}_{}",
        line_command.to_string(),
        generate(RAND_STRING_SIZE, CHARSET)
    );
    let data = LogicalTemplateData {
        random_addr: random_addr,
    };

    let template_name = match line_command {
        LineCommand::Equal => "";
        _ => panic!("Well, this should never happen"),
    };
    String::new()
}
