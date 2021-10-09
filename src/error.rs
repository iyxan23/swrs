use std::io;
use std::fmt::Debug;
use block_modes::BlockModeError;

pub type SWRSResult<T> = Result<T, SWRSError>;

#[derive(Debug)]
pub enum SWRSError {
    DecryptionError(BlockModeError),
    ParseError(ParseError),
    IOError(io::Error),
}

#[derive(Debug)]
pub struct ParseError {
    pub title: String,
    pub description: String,
}

impl ParseError {
    pub fn new(title: &str, desc: &str) -> Self {
        ParseError {
            title: title.to_string(),
            description: desc.to_string()
        }
    }
}