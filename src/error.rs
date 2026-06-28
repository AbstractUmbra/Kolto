use std::io::Error as IoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unable to read file: {0}")]
    Io(#[from] IoError),
    #[error("missing file format header")]
    MissingHeader,
}

pub type Result<T> = std::result::Result<T, Error>;
