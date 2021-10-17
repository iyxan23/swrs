use super::error::SWRSResult;

pub mod project;
pub mod file;
pub mod library;
pub mod resource;
pub(crate) mod serde_util;

/// Represents a project file (file, library, logic, project, etc) that can be parsed from its
/// decrypted content & be reconstructed back into its original string form
pub trait ProjectData {
    /// Parses a decrypted content of itself and returns an instance of itself wrapped around a [`SWRSResult`]
    fn parse(decrypted_content: &str) -> SWRSResult<Self> where Self: Sized;

    /// Reconstructs itself into a string form wrapped around a [`SWRSResult`]
    fn reconstruct(&self) -> SWRSResult<&str>;
}