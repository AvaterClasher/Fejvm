use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClassReaderError {
    #[error("generic I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("invalid class file: {0}")]
    InvalidClassData(String),

    #[error("unsupported class file version {0}")]
    UnsupportedVersion(u16),
}

pub type Result<T> = std::result::Result<T, ClassReaderError>;
