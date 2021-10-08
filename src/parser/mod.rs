pub mod project;
pub mod file;
pub mod library;

#[derive(Debug)]
pub struct RawDecryptedSketchwareProject {
    pub project: String,
    pub file: String,
    pub logic: String,
    pub view: String,
    pub library: String,
    pub resource: String,
}