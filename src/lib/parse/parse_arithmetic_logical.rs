use crate::{error::TranslatorResult, TranslatorError};

use super::{LineCommand, ParsedLine};

pub fn parse_arithmetic_logical(
    line: &String,
    parts: &Vec<&str>,
    command: LineCommand,
    line_no: usize,
) -> TranslatorResult<ParsedLine> {
    if parts.len() != 1 {
        return Err(TranslatorError::SyntaxError {
            line_no: line_no,
            line: line.to_owned(),
            message: "Extra arguments".to_owned(),
        });
    }

    Ok(ParsedLine {
        command,
        memory_segment: None,
        label: None,
        func: None,
        i: None,
    })
}
