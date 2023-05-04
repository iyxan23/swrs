use crate::parser::file::{FileParseError, FileReconstructionError};
use crate::parser::library::{LibraryParseError, LibraryReconstructionError};
use crate::parser::logic::{LogicParseError, LogicReconstructionError};
use crate::parser::resource::{ResourceParseError, ResourceReconstructionError};
use crate::parser::view::{ViewParseError, ViewReconstructionError};
use crate::CryptoError;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

pub mod file;
pub mod library;
pub mod logic;
pub mod project;
pub mod resource;
pub(crate) mod serde_util;
pub mod view;

/// Represents a parsable (and possibly re-construct-able) object
pub trait Parsable
where
    Self: Sized,
{
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
#[derive(Debug, Clone, PartialEq)]
pub struct RawSketchwareProject {
    pub project: String,
    pub file: String,
    pub library: String,
    pub resource: String,
    pub view: String,
    pub logic: String,

    /// A list of resource files that belongs to this project
    ///
    /// `None` means to automatically assign missing resources with a random id
    pub resource_files: Option<Vec<ResourceFileWrapper>>,
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
        resource_files: Vec<ResourceFileWrapper>,
    ) -> Self {
        RawSketchwareProject {
            project,
            file,
            library,
            resource,
            view,
            logic,
            resource_files: Some(resource_files),
        }
    }

    /// Creates a RawSketchwareProject with the specified fields without the resource files, they
    /// will all be assigned to random ids
    pub fn new_wo_res(
        project: String,
        file: String,
        library: String,
        resource: String,
        view: String,
        logic: String,
    ) -> Self {
        RawSketchwareProject {
            project,
            file,
            library,
            resource,
            view,
            logic,
            resource_files: None,
        }
    }

    pub fn from_encrypted(
        project: Vec<u8>,
        file: Vec<u8>,
        library: Vec<u8>,
        resource: Vec<u8>,
        view: Vec<u8>,
        logic: Vec<u8>,
        resource_files: Vec<ResourceFileWrapper>,
    ) -> Result<Self, CryptoError> {
        macro_rules! decrypt {
            ($name_ident:ident, $name:expr) => {
                String::from_utf8(super::decrypt_sw_encrypted(&$name_ident)?)
                    .map_err(CryptoError::FromUtf8Error)?
            };
        }

        Ok(RawSketchwareProject {
            project: decrypt!(project, "project"),
            file: decrypt!(file, "file"),
            library: decrypt!(library, "library"),
            resource: decrypt!(resource, "resource"),
            view: decrypt!(view, "view"),
            logic: decrypt!(logic, "logic"),
            resource_files: Some(resource_files),
        })
    }

    /// Creates a RawSketchwareProject from encrypted sketchware project data without the resource
    /// files, they will all be assigned to random ids
    pub fn from_encrypted_wo_res(
        project: Vec<u8>,
        file: Vec<u8>,
        library: Vec<u8>,
        resource: Vec<u8>,
        view: Vec<u8>,
        logic: Vec<u8>,
    ) -> Result<Self, CryptoError> {
        macro_rules! decrypt {
            ($name_ident:ident, $name:expr) => {
                String::from_utf8(super::decrypt_sw_encrypted(&$name_ident)?)
                    .map_err(CryptoError::FromUtf8Error)?
            };
        }

        Ok(RawSketchwareProject {
            project: decrypt!(project, "project"),
            file: decrypt!(file, "file"),
            library: decrypt!(library, "library"),
            resource: decrypt!(resource, "resource"),
            view: decrypt!(view, "view"),
            logic: decrypt!(logic, "logic"),
            resource_files: None,
        })
    }
}

/// Represents a parsed sketchware project that contains
/// [`project::Project`], [`file::File`], [`library::Library`], [`resource::Resource`],
/// [`view::View`], and [`logic::Logic`]
#[derive(Debug, Clone, PartialEq)]
pub struct SketchwareProject {
    pub project: project::Project,
    pub file: file::File,
    pub library: library::Library,
    pub resource: resource::Resource,
    pub view: view::View,
    pub logic: logic::Logic,

    /// The resource files attached to this project. If None, that means the resource files are
    /// ignored
    pub resource_files: Option<ResourceFiles>,
}

impl SketchwareProject {
    /// Parses a [`RawSketchwareProject`] into [`SketchwareProject`]
    pub fn parse_from(
        raw_swproj: RawSketchwareProject,
    ) -> Result<Self, SketchwareProjectParseError> {
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

            resource_files: raw_swproj
                .resource_files
                .map(|r| r.try_into())
                .transpose()
                .map_err(SketchwareProjectParseError::ResourceFilesParseError)?,
        })
    }

    /// Parses a list of project data into [`SketchwareProject`]
    pub fn parse(
        project: String,
        file: String,
        library: String,
        resource: String,
        view: String,
        logic: String,
        resource_files: Vec<ResourceFileWrapper>,
    ) -> Result<Self, SketchwareProjectParseError> {
        SketchwareProject::parse_from(RawSketchwareProject {
            project,
            file,
            library,
            resource,
            view,
            logic,
            resource_files: Some(resource_files),
        })
    }

    /// Parses a list of project data into [`SketchwareProject`] without resource files (they'll be
    /// ignored on API)
    pub fn parse_wo_res(
        project: String,
        file: String,
        library: String,
        resource: String,
        view: String,
        logic: String,
    ) -> Result<Self, SketchwareProjectParseError> {
        SketchwareProject::parse_from(RawSketchwareProject {
            project,
            file,
            library,
            resource,
            view,
            logic,
            resource_files: None,
        })
    }
}

impl TryInto<RawSketchwareProject> for SketchwareProject {
    type Error = SketchwareProjectReconstructionError;

    fn try_into(self) -> Result<RawSketchwareProject, Self::Error> {
        Ok(RawSketchwareProject {
            project: self
                .project
                .reconstruct()
                .map_err(SketchwareProjectReconstructionError::ProjectReconstructionError)?,

            file: self
                .file
                .reconstruct()
                .map_err(SketchwareProjectReconstructionError::FileReconstructionError)?,

            library: self
                .library
                .reconstruct()
                .map_err(SketchwareProjectReconstructionError::LibraryReconstructionError)?,

            resource: self
                .resource
                .reconstruct()
                .map_err(SketchwareProjectReconstructionError::ResourceReconstructionError)?,

            view: self
                .view
                .reconstruct()
                .map_err(SketchwareProjectReconstructionError::ViewReconstructionError)?,

            logic: self
                .logic
                .reconstruct()
                .map_err(SketchwareProjectReconstructionError::LogicReconstructionError)?,

            resource_files: self.resource_files.map(|r| r.into()),
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
    ResourceFilesParseError(#[from] ResourceFilesParseError),
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

/// A wrapper to a real or imaginary resource file, a "wrapper" that can be either an id (string or
/// u32) or a real file in the filesystem
///
/// If you made an imaginary resource file, make sure for the filename of a file that corresponds
/// to imaginary resource file to have the same name as the id's `res_full_name`
///
/// This enum is made so that swrs is portable and can be used across platforms with very little to
/// no tweaking
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceFileWrapper {
    /// A real path to a real file in the filesystem. swrs will use its path to determine what type
    /// of resource this is, filename as the resource name. and swrs will do a check if this file
    /// exists
    Path(PathBuf),

    /// An imaginary file that is identified with a string
    StringId {
        id: String,

        /// The resource file name, with its extension. This is used to match with the resources
        /// used within this sketchware project.
        ///
        /// please make sure the filename of the file that corresponds to this matches with this
        res_full_name: String,
        res_type: ResourceType,
    },

    /// An imaginary file that is identified with an unsigned 32-bit integer
    U32Id {
        id: u32,

        /// The resource file name, with its extension. This is used to match with the resources
        /// used within this sketchware project
        ///
        /// please make sure the filename of the file that corresponds to this matches with this
        res_full_name: String,
        res_type: ResourceType,
    },
}

impl ResourceFileWrapper {
    pub fn get_full_name(&self) -> String {
        match &self {
            ResourceFileWrapper::Path(path) => {
                path.file_name().unwrap().to_str().unwrap().to_string() /* should never fail */
            }

            ResourceFileWrapper::StringId { res_full_name, .. } => res_full_name.to_owned(),
            ResourceFileWrapper::U32Id { res_full_name, .. } => res_full_name.to_owned(),
        }
    }

    /// Generates a random ResourceFileWrapper::U32Id with the provided resource name and type
    /// using [`rand::random()`].
    #[cfg(feature = "resource_id_random")]
    pub fn make_random_id(res_full_name: String, res_type: ResourceType) -> ResourceFileWrapper {
        ResourceFileWrapper::U32Id {
            id: rand::random(),
            res_full_name,
            res_type,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ResourceType {
    Image,
    Sound,
    Font,
    CustomIcon,
}

/// A struct that stores all the resources of a sketchware project its attached to
///
/// Filled with HashMaps with keys of resource full names
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceFiles {
    pub custom_icon: Option<ResourceFileWrapper>,
    pub images: HashMap<String, ResourceFileWrapper>,
    pub sounds: HashMap<String, ResourceFileWrapper>,
    pub fonts: HashMap<String, ResourceFileWrapper>,
}

impl TryFrom<Vec<ResourceFileWrapper>> for ResourceFiles {
    type Error = ResourceFilesParseError;

    fn try_from(value: Vec<ResourceFileWrapper>) -> Result<Self, Self::Error> {
        let mut images = HashMap::new();
        let mut sounds = HashMap::new();
        let mut fonts = HashMap::new();
        let mut custom_icon = None;

        for path in value {
            match &path {
                ResourceFileWrapper::Path(file) => {
                    if !file.exists() {
                        return Err(ResourceFilesParseError::FileDoesntExist {
                            path: file.clone(),
                        });
                    }

                    // its path should be
                    //
                    // xxx/.sketchware/resources/(images|sounds|fonts|icons)/{project id}/file.extension
                    //
                    // the subfolder after /resources/ determines what type of resource it is
                    let res_type = file
                        .parent()
                        .ok_or_else(|| ResourceFilesParseError::InvalidPath { path: file.clone() })?
                        .parent()
                        .ok_or_else(|| ResourceFilesParseError::InvalidPath { path: file.clone() })?
                        .file_name()
                        .ok_or_else(|| ResourceFilesParseError::InvalidPath { path: file.clone() })?
                        .to_str()
                        .ok_or_else(|| ResourceFilesParseError::InvalidPath {
                            path: file.clone(),
                        })?;

                    let res_full_name = file
                        .file_name()
                        .ok_or_else(|| ResourceFilesParseError::InvalidPath { path: file.clone() })?
                        .to_str()
                        .ok_or_else(|| ResourceFilesParseError::InvalidPath { path: file.clone() })?
                        .to_string();

                    match res_type {
                        "images" => {
                            images.insert(res_full_name, path);
                        }
                        "sounds" => {
                            sounds.insert(res_full_name, path);
                        }
                        "fonts" => {
                            fonts.insert(res_full_name, path);
                        }
                        "icons" => {
                            custom_icon = Some(path);
                        }

                        _ => Err(ResourceFilesParseError::InvalidPath { path: file.clone() })?,
                    }
                }

                ResourceFileWrapper::StringId {
                    res_type,
                    res_full_name,
                    ..
                } => match res_type {
                    ResourceType::Image => {
                        images.insert(res_full_name.to_owned(), path);
                    }
                    ResourceType::Sound => {
                        sounds.insert(res_full_name.to_owned(), path);
                    }
                    ResourceType::Font => {
                        fonts.insert(res_full_name.to_owned(), path);
                    }
                    ResourceType::CustomIcon => {
                        custom_icon = Some(path);
                    }
                },

                ResourceFileWrapper::U32Id {
                    res_type,
                    res_full_name,
                    ..
                } => match res_type {
                    ResourceType::Image => {
                        images.insert(res_full_name.to_owned(), path);
                    }
                    ResourceType::Sound => {
                        sounds.insert(res_full_name.to_owned(), path);
                    }
                    ResourceType::Font => {
                        fonts.insert(res_full_name.to_owned(), path);
                    }
                    ResourceType::CustomIcon => {
                        custom_icon = Some(path);
                    }
                },
            }
        }

        Ok(ResourceFiles {
            custom_icon,
            images,
            sounds,
            fonts,
        })
    }
}

#[derive(Error, Debug)]
pub enum ResourceFilesParseError {
    #[error("file `{path:?}` does not exist")]
    FileDoesntExist { path: PathBuf },
    #[error("path given `{path:?}` is invalid (are you sure its pointing to a sketchware's resources folder?)")]
    InvalidPath { path: PathBuf },
}

impl Into<Vec<ResourceFileWrapper>> for ResourceFiles {
    fn into(self) -> Vec<ResourceFileWrapper> {
        let mut result = Vec::new();

        if let Some(custom_icon) = self.custom_icon {
            result.push(custom_icon);
        }

        result.append(&mut self.images.into_values().collect());
        result.append(&mut self.sounds.into_values().collect());
        result.append(&mut self.fonts.into_values().collect());

        result
    }
}

impl Default for ResourceFiles {
    fn default() -> Self {
        ResourceFiles {
            custom_icon: None,
            images: HashMap::new(),
            sounds: HashMap::new(),
            fonts: HashMap::new(),
        }
    }
}
