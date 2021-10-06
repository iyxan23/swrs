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

pub struct ParseError {
    title: String,
    description: String,
    line: u16,
    column: u16,
    length: u16,
    snippet: u16,
}