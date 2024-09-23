pub mod line_command;
pub mod memory_segment;
pub mod parsed_line;

pub use parsed_line::ParsedLine;
pub use line_command::{LineCommand, LineCommandType};
pub use memory_segment::MemorySegment;

mod parse_memory;
mod parse_arithmetic_logical;
mod parse_branching;

use parse_memory::parse_memory;
use parse_arithmetic_logical::parse_arithmetic_logical;
use parse_branching::parse_branching;
use lazy_static::lazy_static;
use regex::Regex;

use crate::error::{TranslatorResult, TranslatorError};

lazy_static! {
    static ref RE: Regex = Regex::new("//.*").unwrap();
}

/// Takes the raw input file lines as a Vector of Strings
pub fn parse_lines(lines: Vec<String>) -> TranslatorResult<Vec<ParsedLine>> {
    let mut parse_output: Vec<ParsedLine> = Vec::new();

    for i in 0..lines.len() {
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
                    line_no: i + 1,
                    line: lines[i].to_owned(),
                    message: format!("Invalid or Unimplemented Operation: {}", parts[0]),
                });
            }
        };

        let parsed_line = match command.command_type() {
            LineCommandType::Arithmetic | LineCommandType::Logical => parse_arithmetic_logical(&lines[i], &parts, command, i + 1),
            LineCommandType::Memory => parse_memory(&lines[i], &parts, command, i + 1),
            LineCommandType::Branching => parse_branching(&lines[i], &parts, command, i + 1),
        }?;

        parse_output.push(parsed_line);
    }

    Ok(parse_output)
}
