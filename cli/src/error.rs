use thiserror::Error;

use parser::error::ParserError;

#[derive(Error, Debug)]
pub enum IoError {
    #[error("input file must have .cnf extension")]
    DifferentExtension,

    #[error("input file must have some extension")]
    MissingExtension,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("io error: {0}")]
    Io(#[from] IoError),

    #[error("parser error: {0}")]
    Parser(#[from] ParserError),
}
