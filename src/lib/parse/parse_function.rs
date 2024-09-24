use crate::{error::TranslatorResult, TranslatorError};

use super::{LineCommand, ParsedLine};

pub fn parse_function(
    line: &String,
    parts: &Vec<&str>,
    command: LineCommand,
    line_no: usize,
) -> TranslatorResult<ParsedLine> {
    if parts.len() > 3 || parts.len() == 2 {
        return Err(TranslatorError::SyntaxError {
            line_no: line_no,
            line: line.to_owned(),
            message: "Extra or Missing arguments".to_owned(),
        });
    }

    match command {
        LineCommand::Function | LineCommand::Call => {
            let n = parts[2].parse::<usize>()?;
            Ok(ParsedLine {
                command: command,
                func: Some(parts[1].to_owned()),
                i: Some(n),
                memory_segment: None,
                label: None,
            })
        }
        LineCommand::Return => Ok(ParsedLine {
            command: command,
            memory_segment: None,
            label: None,
            func: None,
            i: None,
        }),
        _ => Err(TranslatorError::UnDefinedBehavior),
    }
}
