use super::error::SWRSResult;

pub mod project;
pub mod file;
pub mod library;
pub mod resource;
pub mod view;
pub mod logic;
pub(crate) mod serde_util;

/// Represents a parsable (and possibly re-construct-able) object
pub trait Parsable {
    /// Parses a decrypted content of itself and returns an instance of itself wrapped around a [`SWRSResult`]
    fn parse(decrypted_content: &str) -> SWRSResult<Self> where Self: Sized;

    /// Reconstructs itself into a string form wrapped around a [`SWRSResult`]
    /// by default, if not implemented, this will return [`SWRSError::NotImplementedError`]
    fn reconstruct(&self) -> SWRSResult<String> {
        unimplemented!()
    }
}