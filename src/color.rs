use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Color {
    /// The RGB color is represented as: 0xffRRGGBB
    value: u32,
}

impl Color {
    pub fn parse_hex(txt: &str) -> Result<Self, ParseIntError> {
        let mut hex = String::new();
        // add the transparency
        if txt.len() == 6 {
            hex.push_str("ff");
        }
        hex.push_str(txt);

        Ok(u32::from_str_radix(&hex, 16)?.into())
    }

    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Color {
            value: (0xFFu32 << 24 | (red as u32) << 16 | (green as u32) << 8 | (blue as u32) << 0)
                as u32,
        }
    }

    pub fn from_argb(alpha: u8, red: u8, green: u8, blue: u8) -> Self {
        Color {
            value: ((alpha as u32) << 24
                | (red as u32) << 16
                | (green as u32) << 8
                | (blue as u32) << 0) as u32,
        }
    }

    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.red(), self.green(), self.blue())
    }

    pub fn alpha(&self) -> u8 {
        (self.value >> 24 & 0b111111111) as u8
    }
    pub fn red(&self) -> u8 {
        (self.value >> 16 & 0b111111111) as u8
    }
    pub fn green(&self) -> u8 {
        (self.value >> 8 & 0b111111111) as u8
    }
    pub fn blue(&self) -> u8 {
        (self.value >> 0 & 0b111111111) as u8
    }
}

impl From<u32> for Color {
    fn from(val: u32) -> Self {
        // only get the first 32 bits (8 alpha, 8 red, 8 green, 8 blue)
        Color {
            value: val & 0xffffffff,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("{:#010x}", self.value))
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("{:#010x}", self.value))
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::from_rgb(0, 0, 0)
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.value as i32)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ColorDeserializer;

        impl<'de> Visitor<'de> for ColorDeserializer {
            type Value = Color;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a floating point integer that holds 3 bytes (R, G, B)")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_f64(v as f64)
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_i64(v as i64)
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // this weird conversion is here to preserve the binary formation of the value.
                // the conversion from f64 to u32 right would break the negative value and thus
                // breaks the color
                Ok(Color::from(v as i32 as u32))
            }
        }

        deserializer.deserialize_any(ColorDeserializer)
    }
}
