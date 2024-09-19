use colored::Colorize;
use std::{error::Error, fmt};

pub type ParseResult<T> = Result<T, ParseError>;

pub enum ParseError {
    SyntaxError(usize, String, String),
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SyntaxError(line_no, line, message) => {
                write!(
                    f,
                    "{} {}\n{}",
                    format!("Line {}:", line_no).bold(),
                    line.underline(),
                    message.red().bold(),
                )
            }
        }
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
