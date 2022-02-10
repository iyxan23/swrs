use std::path::PathBuf;
use thiserror::Error;
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

    /// A list of resource files that belongs to this project; yes you need to do some work to get
    /// the files
    ///
    /// These files must match `.sketchware/resources/(fonts|icons|images|sounds)/{project_id}/*`
    /// and must exist
    pub resources: Vec<PathBuf>
}

impl RawSketchwareProject {
    /// Creates a RawSketchwareProject with the specified fields
    pub fn new(
        project: String,
        file: String,
        library: String,
        resource: String,
        view: String,
        logic: String,
        resources: Vec<PathBuf>
    ) -> Self {
        RawSketchwareProject { project, file, library, resource, view, logic, resources }
    }

    pub fn from_encrypted(
        project: Vec<u8>,
        file: Vec<u8>,
        library: Vec<u8>,
        resource: Vec<u8>,
        view: Vec<u8>,
        logic: Vec<u8>,
        resources: Vec<PathBuf>
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
            resources
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
    pub resources: ResourceFiles,
}

impl SketchwareProject {
    /// Parses a [`RawSketchwareProject`] into [`SketchwareProject`]
    pub fn parse_from(raw_swproj: RawSketchwareProject) -> SWRSResult<Self> {
        Ok(SketchwareProject {
            project: project::Project::parse(raw_swproj.project.as_str())
                .map_err(|err|SWRSError::ParseError(format!(
                    "Err while parsing project: {}", err
                )))?,

            file: file::File::parse(raw_swproj.file.as_str())
                .map_err(|err|SWRSError::ParseError(format!(
                    "Err while parsing file: {}", err
                )))?,

            library: library::Library::parse(raw_swproj.library.as_str())
                .map_err(|err|SWRSError::ParseError(format!(
                    "Err while parsing library: {}", err
                )))?,

            resource: resource::Resource::parse(raw_swproj.resource.as_str())
                .map_err(|err|SWRSError::ParseError(format!(
                    "Err while parsing resource: {}", err
                )))?,

            view: view::View::parse(raw_swproj.view.as_str())
                .map_err(|err|SWRSError::ParseError(format!(
                    "Err while parsing view: {}", err
                )))?,

            logic: logic::Logic::parse(raw_swproj.logic.as_str())
                .map_err(|err|SWRSError::ParseError(format!(
                    "Err while parsing logic: {}", err
                )))?,

            resources: raw_swproj.resources.try_into()
                .map_err(|err|SWRSError::ParseError(format!(
                    "Err while retrieving resource files" // todo: show the err once we got thiserror
                )))?,
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
            logic: logic::Logic::parse(logic)?,
            resources: Default::default()
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
            logic: self.logic.reconstruct()?,
            resources: self.resources.into()
        })
    }
}

/// A struct that stores all the resources of a sketchware project its attached to
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceFiles {
    pub custom_icon: Option<PathBuf>,
    pub images: Vec<PathBuf>,
    pub sounds: Vec<PathBuf>,
    pub fonts: Vec<PathBuf>,
}

impl TryFrom<Vec<PathBuf>> for ResourceFiles {
    type Error = ResourceFilesParseError;

    fn try_from(value: Vec<PathBuf>) -> Result<Self, Self::Error> {
        let mut images = vec![];
        let mut sounds = vec![];
        let mut fonts = vec![];
        let mut custom_icon = None;

        for path in value {
            // check if it exists
            if !path.exists() {
                return Err(ResourceFilesParseError::FileDoesntExist { path });
            }

            // put it on different lists based on its category.
            // its path should be
            //
            // xxx/.sketchware/resources/(images|sounds|fonts|icons)/{project id}/file.extension
            //
            // the subfolder after /resources/ determines what type of resource it is
            match path
                .parent().ok_or_else(|| ResourceFilesParseError::InvalidPath { path: path.clone()})?
                .parent().ok_or_else(|| ResourceFilesParseError::InvalidPath { path: path.clone() })?
                .file_name().ok_or_else(|| ResourceFilesParseError::InvalidPath { path: path.clone() })?
                .to_str().ok_or_else(|| ResourceFilesParseError::InvalidPath { path: path.clone() })? {

                "images" => images.push(path),
                "sounds" => sounds.push(path),
                "fonts" => fonts.push(path),
                "icons" => custom_icon = Some(path),

                &_ => Err(ResourceFilesParseError::InvalidPath { path })?
            }
        }

        Ok(ResourceFiles { custom_icon, images, sounds, fonts })
    }
}

#[derive(Error, Debug)]
pub enum ResourceFilesParseError {
    #[error("file `{path:?}` does not exist")]
    FileDoesntExist {
        path: PathBuf
    },
    #[error("path given `{path:?}` is invalid (are you sure its pointing to a sketchware's resources folder?)")]
    InvalidPath {
        path: PathBuf
    }
}

impl Into<Vec<PathBuf>> for ResourceFiles {
    fn into(mut self) -> Vec<PathBuf> {
        let mut result = Vec::new();

        if let Some(custom_icon) = self.custom_icon {
            result.push(custom_icon);
        }

        result.append(&mut self.images);
        result.append(&mut self.sounds);
        result.append(&mut self.fonts);

        result
    }
}

impl Default for ResourceFiles {
    fn default() -> Self {
        ResourceFiles {
            custom_icon: None,
            images: vec![],
            sounds: vec![],
            fonts: vec![]
        }
    }
}