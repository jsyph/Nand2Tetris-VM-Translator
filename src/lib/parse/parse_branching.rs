use crate::{error::TranslatorResult, TranslatorError};

use super::{LineCommand, ParsedLine};

pub fn parse_branching(
    line: &String,
    parts: &Vec<&str>,
    command: LineCommand,
    line_no: usize,
) -> TranslatorResult<ParsedLine> {
    if parts.len() != 2 {
        return Err(TranslatorError::SyntaxError {
            line_no: line_no,
            line: line.to_owned(),
            message: "Extra arguments".to_owned(),
        });
    }

    Ok(ParsedLine {
        command: command,
        memory_segment: None,
        memory_addr: None,
        label: Some(parts[1].to_owned()),
    })
}
