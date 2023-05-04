use std::fmt::{Display, Formatter};
use thiserror::Error;

/// `Error` type for this crate.
///
/// This is a simple `enum` that wraps the `std::io::Error` and `nbt::Error` types.
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("NBT error: {0}")]
    NBT(#[from] nbt::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// `Error` type for parsing.
#[derive(Debug, Error)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error")
    }
}
