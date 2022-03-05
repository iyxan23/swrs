use serde::{Serialize, Deserialize};
use super::Parsable;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Library {
    pub firebase_db: LibraryItem,
    pub compat: LibraryItem,
    pub admob: LibraryItem,
    pub google_map: LibraryItem,
}

impl Parsable for Library {
    type ParseError = LibraryParseError;
    type ReconstructionError = LibraryReconstructionError;

    fn parse(decrypted_content: &str) -> Result<Self, Self::ParseError> {
        let mut newline_iter = decrypted_content.split("\n");

        let mut firebase_db = Option::<LibraryItem>::None;
        let mut compat      = Option::<LibraryItem>::None;
        let mut admob       = Option::<LibraryItem>::None;
        let mut google_map  = Option::<LibraryItem>::None;

        let mut line_count = 0u32;

        macro_rules! library_item_set {
            ($variable:ident, $header:expr) => {
                {
                    $variable = Some(LibraryItem::parse(
                        newline_iter.next()
                            .ok_or_else(|| LibraryParseError::EOFAfterHeader {
                                header: $header.to_string()
                            })?
                    ).map_err(|err| LibraryParseError::LibraryItemParseError {
                        source: err,
                        header: $header.to_string(),
                        line: line_count
                    })?)
                }
            }
        }

        loop {
            let cur_line = newline_iter.next();
            if cur_line.is_none() { break; }
            let cur_line = cur_line.unwrap();

            match cur_line {
                "@firebaseDB" => library_item_set!(firebase_db, "@firebaseDB"),
                "@compat" => library_item_set!(compat, "@compat"),
                "@admob" => library_item_set!(admob, "@admob"),
                "@googleMap" => library_item_set!(google_map, "@googleMap"),
                _ => ()
            }

            line_count += 1;
        }

        Ok(Library {
            firebase_db: firebase_db.ok_or_else(||LibraryParseError::MissingItem { header: "@firebaseDB".to_string() })?,
            compat: compat.ok_or_else(||LibraryParseError::MissingItem { header: "@compat".to_string() })?,
            admob: admob.ok_or_else(||LibraryParseError::MissingItem { header: "@admob".to_string() })?,
            google_map: google_map.ok_or_else(||LibraryParseError::MissingItem { header: "@googleMap".to_string() })?,
        })
    }

    fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
        Ok(format!(
            "@firebaseDB\n{}\n@compat\n{}\n@admob\n{}\n@googleMap\n{}",
            // a bit messy i know, some macros can be handy
            self.firebase_db.reconstruct()
                .map_err(|err| LibraryReconstructionError::LibraryItemReconstructionError {
                    source: err,
                    header: "@firebaseDB".to_string()
                })?,
            self.compat.reconstruct()
                .map_err(|err| LibraryReconstructionError::LibraryItemReconstructionError {
                    source: err,
                    header: "@compat".to_string()
                })?,
            self.admob.reconstruct()
                .map_err(|err| LibraryReconstructionError::LibraryItemReconstructionError {
                    source: err,
                    header: "@admob".to_string()
                })?,
            self.google_map.reconstruct()
                .map_err(|err| LibraryReconstructionError::LibraryItemReconstructionError {
                    source: err,
                    header: "@googleMap".to_string()
                })?,
        ))
    }
}

#[derive(Error, Debug)]
pub enum LibraryParseError {
    #[error("end of file after the header {header}")]
    EOFAfterHeader { header: String },

    #[error("error while parsing a library item of {header} at line {line}")]
    LibraryItemParseError {
        #[source]
        source: serde_json::Error,
        header: String,
        line: u32
    },

    #[error("missing a library item of {header}")]
    MissingItem {
        header: String
    }
}

#[derive(Error, Debug)]
pub enum LibraryReconstructionError {
    #[error("error while reconstructing the library item of {header}")]
    LibraryItemReconstructionError {
        #[source]
        source: serde_json::Error,
        header: String
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
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
    type ParseError = serde_json::Error;
    type ReconstructionError = serde_json::Error;

    fn parse(decrypted_content: &str) -> Result<Self, Self::ParseError> {
        serde_json::from_str(decrypted_content)
    }

    fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
        serde_json::to_string(self)
    }
}

/// A struct that represents an ad unit; Ad units are containers you place in your apps to show ads
/// to users (source: <https://support.google.com/admob/answer/6128738?hl=en>)
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct AdUnit {
    pub id: String,
    pub name: String,
}