mod parse;
mod generation;
mod error;

pub use parse::parse_lines;
pub use generation::generate_code;
pub use error::TranslatorError;
