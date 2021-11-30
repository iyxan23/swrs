use serde::{Serialize, Deserialize};
use crate::error::{SWRSError, SWRSResult};
use super::Parsable;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Library {
    pub firebase_db: LibraryItem,
    pub compat: LibraryItem,
    pub admob: LibraryItem,
    pub google_map: LibraryItem,
}

impl Parsable for Library {
    fn parse(decrypted_content: &str) -> SWRSResult<Self> {
        let mut newline_iter = decrypted_content.split("\n");

        let mut firebase_db = Option::<LibraryItem>::None;
        let mut compat      = Option::<LibraryItem>::None;
        let mut admob       = Option::<LibraryItem>::None;
        let mut google_map  = Option::<LibraryItem>::None;

        loop {
            let line = newline_iter.next();
            if line.is_none() { break; }
            let line = line.unwrap();

            match line {
                "@firebaseDB" => {
                    firebase_db = Some(LibraryItem::parse(
                        newline_iter.next()
                            .ok_or_else(||SWRSError::ParseError("Couldn't get firebaseDB's library information".to_string()))?
                    )?);
                }

                "@compat" => {
                    compat = Some(LibraryItem::parse(
                        newline_iter.next()
                            .ok_or_else(||SWRSError::ParseError("Couldn't get compat's library information".to_string()))?
                    )?);
                }

                "@admob" => {
                    admob = Some(LibraryItem::parse(
                        newline_iter.next()
                            .ok_or_else(||SWRSError::ParseError("Couldn't get admob's library information".to_string()))?
                    )?);
                }

                "@googleMap" => {
                    google_map = Some(LibraryItem::parse(
                        newline_iter.next()
                            .ok_or_else(||SWRSError::ParseError("Couldn't get googleMap's library information".to_string()))?
                    )?);
                }
                _ => ()
            }
        }

        Ok(Library {
            firebase_db: firebase_db.ok_or_else(||SWRSError::ParseError("Cannot find firebaseDB's library information".to_string()))?,
            compat: compat.ok_or_else(||SWRSError::ParseError("Cannot find compat's library information".to_string()))?,
            admob: admob.ok_or_else(||SWRSError::ParseError("Cannot find admob's library information".to_string()))?,
            google_map: google_map.ok_or_else(||SWRSError::ParseError("Cannot find googlemap's library information".to_string()))?,
        })
    }

    fn reconstruct(&self) -> SWRSResult<String> {
        Ok(format!(
            "@firebaseDB\n{}\n@compat\n{}\n@admob\n{}\n@googleMap\n{}",
            self.firebase_db.reconstruct()?,
            self.compat.reconstruct()?,
            self.admob.reconstruct()?,
            self.google_map.reconstruct()?,
        ))
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LibraryItem {
    pub ad_units: Vec<AdUnit>,
    pub data: String,
    pub lib_type: u8,
    pub reserved1: String,
    pub reserved2: String,
    pub reserved3: String,
    pub test_devices: Vec<String>,
    pub use_yn: String,
}

impl Parsable for LibraryItem {
    fn parse(decrypted_content: &str) -> SWRSResult<Self> where Self: Sized {
        serde_json::from_str(decrypted_content)
            .map_err(|e|SWRSError::ParseError(e.to_string()))
    }

    fn reconstruct(&self) -> SWRSResult<String> {
        serde_json::to_string(self)
            .map_err(|e|SWRSError::ReconstructionError(e.to_string()))
    }
}

/// A struct that represents an ad unit; Ad units are containers you place in your apps to show ads
/// to users (source: https://support.google.com/admob/answer/6128738?hl=en)
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AdUnit {
    pub id: String,
    pub name: String,
}