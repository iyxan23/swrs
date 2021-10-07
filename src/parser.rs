use crate::error::SWRSResult;

#[derive(Debug)]
pub struct Project {
    pub project_id: u16,
    pub app_name: String,
    pub package_name: String,
    pub workspace_name: String,
    pub version_code: u8,
    pub version_name: String,
    pub colors: Colors,
    pub create_date: u64,
    pub sketchware_version: u8,
}

#[derive(Debug)]
pub struct Colors {
    pub primary: Color,
    pub primary_dark: Color,
    pub accent: Color,
    pub control_normal: Color,
    pub control_highlight: Color,
}

#[derive(Debug)]
pub struct Color {
    /// The RGB color is represented as: 0xffRRGGBB
    value: u32
}

impl Color {
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Color { value: (0xFF << 24 | red << 16 | green << 8 | blue << 0) as u32 }
    }

    pub fn rgb(&self) -> (u8, u8, u8) { (self.red(), self.green(), self.blue()) }

    pub fn red(&self) -> u8 { (self.value >> 16 & 0b111111111) as u8 }
    pub fn green(&self) -> u8 { (self.value >> 8 & 0b111111111) as u8 }
    pub fn blue(&self) -> u8 { (self.value >> 0 & 0b111111111) as u8 }
}

impl From<u32> for Color {
    fn from(val: u32) -> Self {
        Color { value: val }
    }
}

#[derive(Debug)]
pub struct RawDecryptedSketchwareProject {
    pub project: String,
    pub file: String,
    pub logic: String,
    pub view: String,
    pub library: String,
    pub resource: String,
}