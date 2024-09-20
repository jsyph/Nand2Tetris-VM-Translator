pub mod line_command;
pub mod memory_segment;
pub mod parsed_line;

pub use parsed_line::ParsedLine;

use lazy_static::lazy_static;
use line_command::{LineCommand, LineCommandType};
use memory_segment::MemorySegment;
use regex::Regex;

use crate::{error::TranslatorResult, TranslatorError};

lazy_static! {
    static ref RE: Regex = Regex::new("//.*").unwrap();
}

/// Takes the raw input file lines as a Vector of Strings
pub fn parse_lines(lines: Vec<String>) -> TranslatorResult<Vec<ParsedLine>> {
    let mut res: Vec<ParsedLine> = Vec::new();

    for i in 1..lines.len() {
        // remove comments and empty lines
        let line_without_comment = RE.replace_all(&lines[i], "").trim().to_owned();
        if line_without_comment.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line_without_comment.split(" ").collect();
        let command_parse_result = LineCommand::parse(parts[0]);
        let command = match command_parse_result {
            Some(res) => res,
            None => {
                return Err(TranslatorError::SyntaxError {
                    line_no: i,
                    line: lines[i].to_owned(),
                    message: format!("Invalid or Unimplemented Operation: {}", parts[0]),
                });
            }
        };

        if let LineCommandType::Memory = command.command_type() {
            if parts.len() < 3 {
                return Err(TranslatorError::SyntaxError {
                    line_no: i,
                    line: lines[i].to_owned(),
                    message: "Missing Arguments".to_owned(),
                });
            }

            let memory_segment_parse_result = MemorySegment::parse(parts[1]);
            let memory_segment = match memory_segment_parse_result {
                Some(res) => res,
                None => {
                    return Err(TranslatorError::SyntaxError {
                        line_no: i,
                        line: lines[i].to_owned(),
                        message: format!("Invalid Memory Segment: {}", parts[1]),
                    });
                }
            };

            // pop cant be with constant memory segment
            if let (MemorySegment::Constant, LineCommand::Pop) = (&memory_segment, &command) {
                return Err(TranslatorError::SyntaxError {
                    line_no: i,
                    line: lines[i].to_owned(),
                    message: format!("Invalid pop argument: constant"),
                });
            }

            let memory_addr_parse_result = parts[2].parse::<usize>();
            let memory_addr = match memory_addr_parse_result {
                Ok(res) => res,
                Err(_) => {
                    return Err(TranslatorError::SyntaxError {
                        line_no: i,
                        line: lines[i].to_owned(),
                        message: format!("Invalid Memory Address: {}", parts[2]),
                    });
                }
            };

            res.push(ParsedLine {
                command: command,
                memory_segment: Some(memory_segment),
                memory_addr: Some(memory_addr),
            });
            continue;
        }

        res.push(ParsedLine {
            command: command,
            memory_segment: None,
            memory_addr: None,
        });
    }

    Ok(res)
}
