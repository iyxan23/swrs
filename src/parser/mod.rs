use crate::error::SWRSError;
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
    /// by default, if not implemented, this will panic ([`unimplemented!()`])
    fn reconstruct(&self) -> SWRSResult<String> {
        unimplemented!()
    }
}

/// Represents a raw (un-parsed) sketchware project
pub struct RawSketchwareProject {
    pub project: String,
    pub file: String,
    pub library: String,
    pub resource: String,
    pub view: String,
    pub logic: String,
}

impl RawSketchwareProject {
    /// Creates a RawSketchwareProject with the specified fields
    pub fn new(project: String, file: String, library: String, resource: String, view: String, logic: String) -> Self {
        RawSketchwareProject { project, file, library, resource, view, logic }
    }

    pub fn from_encrypted(
        project: Vec<u8>,
        file: Vec<u8>,
        library: Vec<u8>,
        resource: Vec<u8>,
        view: Vec<u8>,
        logic: Vec<u8>
    ) -> SWRSResult<Self> {
        macro_rules! decrypt {
            ($name_ident:ident, $name:expr) => {
                String::from_utf8(super::decrypt_sw_encrypted(&$name_ident)?)
                    .map_err(|e| SWRSError::ParseError(format!(
                        "Failed to decode {} due to an encoding error: {}", $name, e
                    )))?
            }
        }

        Ok(RawSketchwareProject {
            project: decrypt!(project, "project"),
            file: decrypt!(file, "file"),
            library: decrypt!(library, "library"),
            resource: decrypt!(resource, "resource"),
            view: decrypt!(view, "view"),
            logic: decrypt!(logic, "logic"),
        })
    }
}

/// Represents a parsed sketchware project that contains
/// [`project::Project`], [`file::File`], [`library::Library`], [`resource::Resource`],
/// [`view::View`], and [`logic::Logic`]
pub struct SketchwareProject {
    pub project: project::Project,
    pub file: file::File,
    pub library: library::Library,
    pub resource: resource::Resource,
    pub view: view::View,
    pub logic: logic::Logic,
}

impl SketchwareProject {
    /// Parses a [`RawSketchwareProject`] into [`SketchwareProject`]
    pub fn parse_from(raw_swproj: RawSketchwareProject) -> SWRSResult<Self> {
        Ok(SketchwareProject {
            project: project::Project::parse(raw_swproj.project.as_str())?,
            file: file::File::parse(raw_swproj.file.as_str())?,
            library: library::Library::parse(raw_swproj.library.as_str())?,
            resource: resource::Resource::parse(raw_swproj.resource.as_str())?,
            view: view::View::parse(raw_swproj.view.as_str())?,
            logic: logic::Logic::parse(raw_swproj.logic.as_str())?
        })
    }

    /// Parses a list of project data into [`SketchwareProject`]
    pub fn parse(project: &str, file: &str, library: &str, resource: &str, view: &str, logic: &str) -> SWRSResult<Self> {
        Ok(SketchwareProject {
            project: project::Project::parse(project)?,
            file: file::File::parse(file)?,
            library: library::Library::parse(library)?,
            resource: resource::Resource::parse(resource)?,
            view: view::View::parse(view)?,
            logic: logic::Logic::parse(logic)?
        })
    }
}

impl TryInto<RawSketchwareProject> for SketchwareProject {
    type Error = SWRSError;

    fn try_into(self) -> Result<RawSketchwareProject, Self::Error> {
        Ok(RawSketchwareProject {
            project: self.project.reconstruct()?,
            file: self.file.reconstruct()?,
            library: self.library.reconstruct()?,
            resource: self.resource.reconstruct()?,
            view: self.view.reconstruct()?,
            logic: self.logic.reconstruct()?
        })
    }
}