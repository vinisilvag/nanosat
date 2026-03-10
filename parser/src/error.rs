use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("failed to open input file")]
    FailedToOpenFile,

    #[error("failed to recover line content")]
    FailedToRecoverLine,

    #[error("CNF file ended with unterminated clause (missing 0)")]
    UnterminatedClause,

    #[error("invalid DIMACS header")]
    InvalidHeader,

    #[error("invalid DIMACS clause")]
    InvalidClause,
}
