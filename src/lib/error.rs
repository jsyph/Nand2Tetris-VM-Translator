use colored::Colorize;
use handlebars::RenderError;
use std::{error::Error, fmt, io, num::ParseIntError};

pub type TranslatorResult<T> = Result<T, TranslatorError>;

pub enum TranslatorError {
    SyntaxError {
        line_no: usize,
        line: String,
        message: String,
    },
    RenderError {
        template_name: Option<String>,
        line_no: Option<usize>,
        column_no: Option<usize>,
    },
    FileIOError {
        message: String,
    },
    InvalidUnignedInt {
        error: ParseIntError,
    },
    UnDefinedBehavior,
}

impl Error for TranslatorError {}

impl fmt::Display for TranslatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            TranslatorError::SyntaxError {
                line_no,
                line,
                message,
            } => format!("Syntax Error on line {}:\n\t{}\n{}", line_no, line, message),
            TranslatorError::RenderError {
                template_name,
                line_no,
                column_no,
            } => format!(
                "Render Error of '{:?}'. On line {:?} | line {} @ {:?}",
                template_name,
                line_no,
                line!(),
                column_no
            ),
            TranslatorError::UnDefinedBehavior => {
                format!("Undefined behavior on line {}", line!())
            }
            TranslatorError::FileIOError { message } => {
                format!("File IO Error on line {}: {}", line!(), message)
            }
            TranslatorError::InvalidUnignedInt { error } => {
                format!("Invalid Unsigned Int Error : {}", error)
            }
        }
        .bold()
        .red();

        write!(f, "{}", message)
    }
}

impl fmt::Debug for TranslatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<RenderError> for TranslatorError {
    fn from(value: RenderError) -> Self {
        Self::RenderError {
            template_name: value.template_name,
            line_no: value.line_no,
            column_no: value.column_no,
        }
    }
}

impl From<io::Error> for TranslatorError {
    fn from(value: io::Error) -> Self {
        Self::FileIOError {
            message: value.to_string(),
        }
    }
}

impl From<ParseIntError> for TranslatorError {
    fn from(value: ParseIntError) -> Self {
        Self::InvalidUnignedInt {
            error: value,
        }
    }
}
