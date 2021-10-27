use std::io;
use std::fmt::{Debug, Display, Formatter};
use block_modes::BlockModeError;

pub type SWRSResult<T> = Result<T, SWRSError>;

#[derive(Debug)]
pub enum SWRSError {
    DecryptionError(BlockModeError),
    IOError(io::Error),
    ParseError(String),
    NotImplementedError,
}

impl Display for SWRSError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SWRSError::DecryptionError(err) => {
                write!(f, "decryption error: {}", err)
            }
            SWRSError::IOError(ioerr) => {
                write!(f, "io error: {}", ioerr)
            }
            SWRSError::ParseError(msg) => {
                write!(f, "parse error: {}", msg)
            }
            SWRSError::NotImplementedError => {
                write!(f, "not implemented error")
            }
        }
    }
}