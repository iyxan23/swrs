use std::collections::HashMap;
use std::path::PathBuf;
use std::string::FromUtf8Error;
use thiserror::Error;
use crate::parser::file::{FileParseError, FileReconstructionError};
use crate::parser::library::{LibraryParseError, LibraryReconstructionError};
use crate::parser::logic::{LogicParseError, LogicReconstructionError};
use crate::parser::resource::{ResourceParseError, ResourceReconstructionError};
use crate::parser::view::{ViewParseError, ViewReconstructionError};

pub mod project;
pub mod file;
pub mod library;
pub mod resource;
pub mod view;
pub mod logic;
pub(crate) mod serde_util;

/// Represents a parsable (and possibly re-construct-able) object
pub trait Parsable
where Self: Sized {
    type ParseError;
    type ReconstructionError;

    /// Parses a decrypted content of itself and returns an instance of itself wrapped around a [`Result`]
    fn parse(decrypted_content: &str) -> Result<Self, Self::ParseError>;

    /// Reconstructs itself into a string form wrapped around a [`Result`]
    /// by default, if not implemented, this will panic ([`unimplemented!()`])
    fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
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
    ) -> Result<Self, FromUtf8Error> {
        macro_rules! decrypt {
            ($name_ident:ident, $name:expr) => {
                String::from_utf8(super::decrypt_sw_encrypted(&$name_ident)?)?
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
    pub resource_files: ResourceFiles,
}

impl SketchwareProject {
    /// Parses a [`RawSketchwareProject`] into [`SketchwareProject`]
    pub fn parse_from(raw_swproj: RawSketchwareProject) -> Result<Self, SketchwareProjectParseError> {
        Ok(SketchwareProject {
            project: project::Project::parse(raw_swproj.project.as_str())
                .map_err(SketchwareProjectParseError::ProjectParseError)?,

            file: file::File::parse(raw_swproj.file.as_str())
                .map_err(SketchwareProjectParseError::FileParseError)?,

            library: library::Library::parse(raw_swproj.library.as_str())
                .map_err(SketchwareProjectParseError::LibraryParseError)?,

            resource: resource::Resource::parse(raw_swproj.resource.as_str())
                .map_err(SketchwareProjectParseError::ResourceParseError)?,

            view: view::View::parse(raw_swproj.view.as_str())
                .map_err(SketchwareProjectParseError::ViewParseError)?,

            logic: logic::Logic::parse(raw_swproj.logic.as_str())
                .map_err(SketchwareProjectParseError::LogicParseError)?,

            resource_files: raw_swproj.resources.try_into()
                .map_err(SketchwareProjectParseError::ResourceFilesParseError)?,
        })
    }

    /// Parses a list of project data into [`SketchwareProject`]
    pub fn parse(project: &str, file: &str, library: &str, resource: &str, view: &str, logic: &str) -> Result<Self, SketchwareProjectParseError> {
        SketchwareProject::parse_from(RawSketchwareProject {
            project: project.to_string(),
            file: file.to_string(),
            library: library.to_string(),
            resource: resource.to_string(),
            view: view.to_string(),
            logic: logic.to_string(),
            resources: vec![]
        })
    }
}

impl TryInto<RawSketchwareProject> for SketchwareProject {
    type Error = SketchwareProjectReconstructionError;

    fn try_into(self) -> Result<RawSketchwareProject, Self::Error> {
        Ok(RawSketchwareProject {
            project: self.project.reconstruct()
                .map_err(SketchwareProjectReconstructionError::ProjectReconstructionError)?,

            file: self.file.reconstruct()
                .map_err(SketchwareProjectReconstructionError::FileReconstructionError)?,

            library: self.library.reconstruct()
                .map_err(SketchwareProjectReconstructionError::LibraryReconstructionError)?,

            resource: self.resource.reconstruct()
                .map_err(SketchwareProjectReconstructionError::ResourceReconstructionError)?,

            view: self.view.reconstruct()
                .map_err(SketchwareProjectReconstructionError::ViewReconstructionError)?,

            logic: self.logic.reconstruct()
                .map_err(SketchwareProjectReconstructionError::LogicReconstructionError)?,

            resources: self.resource_files.into()
        })
    }
}

#[derive(Error, Debug)]
pub enum SketchwareProjectParseError {
    #[error("failed to parse the data file `project`")]
    ProjectParseError(#[from] serde_json::Error),

    #[error("failed to parse the data file `file`")]
    FileParseError(#[from] FileParseError),

    #[error("failed to parse the data file `library`")]
    LibraryParseError(#[from] LibraryParseError),

    #[error("failed to parse the data file `resource`")]
    ResourceParseError(#[from] ResourceParseError),

    #[error("failed to parse the data file `view`")]
    ViewParseError(#[from] ViewParseError),

    #[error("failed to parse the data file `logic`")]
    LogicParseError(#[from] LogicParseError),

    #[error("failed retrieve resource files")]
    ResourceFilesParseError(#[from] ResourceFilesParseError)
}

// these names might be too long lol, should i shorten them to something like SwProjectReconError?
#[derive(Error, Debug)]
pub enum SketchwareProjectReconstructionError {
    #[error("failed to reconstruct the data file `project`")]
    ProjectReconstructionError(#[from] serde_json::Error),

    #[error("failed to reconstruct the data file `file`")]
    FileReconstructionError(#[from] FileReconstructionError),

    #[error("failed to reconstruct the data file `library`")]
    LibraryReconstructionError(#[from] LibraryReconstructionError),

    #[error("failed to reconstruct the data file `resource`")]
    ResourceReconstructionError(#[from] ResourceReconstructionError),

    #[error("failed to reconstruct the data file `view`")]
    ViewReconstructionError(#[from] ViewReconstructionError),

    #[error("failed to reconstruct the data file `logic`")]
    LogicReconstructionError(#[from] LogicReconstructionError),
}

/// A wrapper to a real or imaginary file, a "wrapper" that can be either an id (string or u32) or
/// a real file in the filesystem
///
/// This enum is made so that swrs is portable and can be used across platforms with very little to
/// no tweaking
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileWrapper {
    /// A real path to a real file in the filesystem. swrs will use its path to determine what type
    /// of resource this is, filename as the resource name. and swrs will do a check if this file
    /// exists
    Path(PathBuf),

    /// An imaginary file that is identified with a string
    StringId {
        id: String,

        /// The resource name, will be used to match the ones used in a sketchware project, must
        /// contain a file extension
        res_name: String,
        res_type: ResourceType
    },

    /// An imaginary file that is identified with an unsigned 32-bit integer
    U32Id {
        id: u32,

        /// The resource name, will be used to match the ones used in a sketchware project, must
        /// contain a file extension
        res_name: String,
        res_type: ResourceType
    },
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResourceType { Image, Sound, Font, CustomIcon }

/// A struct that stores all the resources of a sketchware project its attached to
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceFiles {
    pub custom_icon: Option<FileWrapper>,
    pub images: HashMap<String, FileWrapper>,
    pub sounds: HashMap<String, FileWrapper>,
    pub fonts: HashMap<String, FileWrapper>,
}

impl TryFrom<Vec<FileWrapper>> for ResourceFiles {
    type Error = ResourceFilesParseError;

    fn try_from(value: Vec<FileWrapper>) -> Result<Self, Self::Error> {
        let mut images = HashMap::new();
        let mut sounds = HashMap::new();
        let mut fonts = HashMap::new();
        let mut custom_icon = None;

        for path in value {
            match &path {
                FileWrapper::Path(file) => {
                    if !file.exists() {
                        return Err(ResourceFilesParseError::FileDoesntExist { path: file.clone() });
                    }

                    // its path should be
                    //
                    // xxx/.sketchware/resources/(images|sounds|fonts|icons)/{project id}/file.extension
                    //
                    // the subfolder after /resources/ determines what type of resource it is
                    let res_type = file
                        .parent().ok_or_else(|| ResourceFilesParseError::InvalidPath { path: file.clone() })?
                        .parent().ok_or_else(|| ResourceFilesParseError::InvalidPath { path: file.clone() })?
                        .file_name().ok_or_else(|| ResourceFilesParseError::InvalidPath { path: file.clone() })?
                        .to_str().ok_or_else(|| ResourceFilesParseError::InvalidPath { path: file.clone() })?;

                    let res_name = file.file_name()
                        .ok_or_else(|| ResourceFilesParseError::InvalidPath { path: file.clone() })?
                        .to_str().ok_or_else(|| ResourceFilesParseError { path: file.clone() })?
                        .to_string();

                    match res_type {
                        "images" => images.insert(res_name, path),
                        "sounds" => sounds.insert(res_name, path),
                        "fonts" => fonts.insert(res_name, path),
                        "icons" => custom_icon = Some(path),

                        &_ => Err(ResourceFilesParseError::InvalidPath { path: file.clone() })?
                    }
                }

                FileWrapper::StringId { res_type, res_name, .. } => {
                    match res_type {
                        ResourceType::Image => images.insert(res_name.to_owned(), path),
                        ResourceType::Sound => sounds.insert(res_name.to_owned(), path),
                        ResourceType::Font => fonts.insert(res_name.to_owned(), path),
                        ResourceType::CustomIcon => { custom_icon = Some(path) }
                    }
                }

                FileWrapper::U32Id { res_type, res_name, .. } => {
                    match res_type {
                        ResourceType::Image => images.insert(res_name.to_owned(), path),
                        ResourceType::Sound => sounds.insert(res_name.to_owned(), path),
                        ResourceType::Font => fonts.insert(res_name.to_owned(), path),
                        ResourceType::CustomIcon => { custom_icon = Some(path) }
                    }
                }
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

impl Into<Vec<FileWrapper>> for ResourceFiles {
    fn into(mut self) -> Vec<FileWrapper> {
        let mut result = Vec::new();

        if let Some(custom_icon) = self.custom_icon {
            result.push(custom_icon);
        }

        result.append(self.images.values().collect());
        result.append(self.sounds.values().collect());
        result.append(self.fonts.values().collect());

        result
    }
}

impl Default for ResourceFiles {
    fn default() -> Self {
        ResourceFiles {
            custom_icon: None,
            images: HashMap::new(),
            sounds: HashMap::new(),
            fonts: HashMap::new()
        }
    }
}