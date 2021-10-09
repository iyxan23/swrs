pub mod project;
pub mod file;
pub mod library;
pub mod resource;
pub mod view;

#[derive(Debug)]
pub struct RawDecryptedSketchwareProject {
    pub project: String,
    pub file: String,
    pub logic: String,
    pub view: String,
    pub library: String,
    pub resource: String,
}