use crate::{error::TranslatorResult, TranslatorError};

use super::{LineCommand, MemorySegment, ParsedLine};

pub fn parse_memory(
    line: &String,
    parts: &Vec<&str>,
    command: LineCommand,
    line_no: usize,
) -> TranslatorResult<ParsedLine> {
    if parts.len() != 3 {
        return Err(TranslatorError::SyntaxError {
            line_no: line_no,
            line: line.to_owned(),
            message: "Missing Arguments".to_owned(),
        });
    }

    let memory_segment_parse_result = MemorySegment::parse(parts[1]);
    let memory_segment = match memory_segment_parse_result {
        Some(res) => res,
        None => {
            return Err(TranslatorError::SyntaxError {
                line_no: line_no,
                line: line.to_owned(),
                message: format!("Invalid Memory Segment: {}", parts[1]),
            });
        }
    };

    // pop cant be with constant memory segment
    if let (MemorySegment::Constant, LineCommand::Pop) = (&memory_segment, &command) {
        return Err(TranslatorError::SyntaxError {
            line_no: line_no,
            line: line.to_owned(),
            message: format!("Invalid pop argument: constant"),
        });
    }

    let memory_addr_parse_result = parts[2].parse::<usize>();
    let memory_addr = match memory_addr_parse_result {
        Ok(res) => res,
        Err(_) => {
            return Err(TranslatorError::SyntaxError {
                line_no: line_no,
                line: line.to_owned(),
                message: format!("Invalid Memory Address: {}", parts[2]),
            });
        }
    };

    Ok(ParsedLine {
        command: command,
        memory_segment: Some(memory_segment),
        memory_addr: Some(memory_addr),
        label: None,
    })
}
