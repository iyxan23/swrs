use std::fmt::{Debug, Display, Formatter};

#[derive(Eq, PartialEq)]
pub struct Color {
    /// The RGB color is represented as: 0xffRRGGBB
    value: u32
}

impl Color {
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Color { value: (0xFFu32 << 24 | (red as u32) << 16 | (green as u32) << 8 | (blue as u32) << 0) as u32 }
    }

    pub fn rgb(&self) -> (u8, u8, u8) { (self.red(), self.green(), self.blue()) }

    pub fn red(&self) -> u8 { (self.value >> 16 & 0b111111111) as u8 }
    pub fn green(&self) -> u8 { (self.value >> 8 & 0b111111111) as u8 }
    pub fn blue(&self) -> u8 { (self.value >> 0 & 0b111111111) as u8 }
}

impl From<u32> for Color {
    fn from(val: u32) -> Self {
        // only get the first 24 bits (8 red, 8 green, 8 blue)
        Color { value: (0xFFu32 << 24) | val & 0xffffff }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("{:#08x}", self.value & 0xffffff))
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("{:#08x}", self.value & 0xffffff))
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::from_rgb(0, 0, 0)
    }
}