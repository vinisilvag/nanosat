//! The types for parser errors.

use thiserror::Error;

/// The error type for the parser.
#[derive(Debug, Error)]
pub enum ParserError {}
