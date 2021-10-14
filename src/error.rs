use std::io;
use std::fmt::Debug;
use block_modes::BlockModeError;

pub type SWRSResult<T> = Result<T, SWRSError>;

#[derive(Debug)]
pub enum SWRSError {
    DecryptionError(BlockModeError),
    IOError(io::Error),
    ParseError(String)
}