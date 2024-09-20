use handlebars::RenderError;
use std::{error::Error, fmt, io};

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
    UnDefinedBehavior,
}

impl Error for TranslatorError {}

impl fmt::Display for TranslatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranslatorError::SyntaxError {
                line_no,
                line,
                message,
            } => write!(
                f,
                "Syntax Error on line {}:\n\t{}\n{}",
                line_no, line, message
            ),
            TranslatorError::RenderError {
                template_name,
                line_no,
                column_no,
            } => write!(
                f,
                "Render Error of '{:?}'. On line {:?} | {} @ {:?}",
                template_name,
                line_no,
                line!(),
                column_no
            ),
            TranslatorError::UnDefinedBehavior => {
                write!(f, "Undefined behavior on line {}", line!())
            }
            TranslatorError::FileIOError { message } => {
                write!(f, "File IO Error on line {}: {}", line!(), message)
            }
        }
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
