use core::str::Split;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use serde_json::Value;
use crate::error::{SWRSError, SWRSResult};
use super::Parsable;

#[derive(Debug, Eq, PartialEq)]
pub struct Library {
    pub firebase_db: Option<FirebaseDB>,
    pub admob: Option<AdMob>,
    pub google_map: Option<GoogleMap>,
    pub appcompat_enabled: bool,
}

impl Parsable for Library {
    fn parse(library: &str) -> SWRSResult<Library> {
        let mut iterator = library.split("\n");
        let mut result = Library {
            firebase_db: None,
            admob: None,
            google_map: None,
            appcompat_enabled: false
        };

        loop {
            let line = iterator.next();
            if line.is_none() { break; }
            let line = line.unwrap();

            fn parse_lib_data<'a, T>(iterator: &mut Split<'a, &str>, cur_line: &str) -> SWRSResult<Option<T>>
            where T: DeserializeOwned {
                let data = iterator.next();
                if data.is_none() {
                    return Err(
                        SWRSError::ParseError(
                            format!("EOF whilst trying to parse the data of {}", cur_line)
                        )
                    );
                }

                let value: Value = serde_json::from_str(data.unwrap())
                    .map_err(|e|SWRSError::ParseError(e.to_string()))?;

                if value["useYn"] == "Y" {
                    serde_json::from_value(value)
                        .map_err(|e|SWRSError::ParseError(e.to_string()))
                        .map(|r|Option::Some(r))
                } else {
                    Ok(None)
                }
            }

            match line {
                "@firebaseDB" => { result.firebase_db = parse_lib_data(&mut iterator, line)?; }
                "@compat" => {
                    let data = iterator.next();
                    if data.is_none() {
                        return Err(
                            SWRSError::ParseError("EOF whilst trying to parse the data of @compat".to_string())
                        )
                    }

                    let value: Value = serde_json::from_str(data.unwrap())
                        .map_err(|e|SWRSError::ParseError(e.to_string()))?;

                    if value["useYn"] == "Y" {
                        result.appcompat_enabled = true;
                    }
                }
                "@admob" => { result.admob = parse_lib_data(&mut iterator, line)?; }
                "@googleMap" => { result.google_map = parse_lib_data(&mut iterator, line)?; }
                &_ => {}
            }
        }

        Ok(result)
    }

    fn reconstruct(&self) -> SWRSResult<String> {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct FirebaseDB {
    #[serde(rename = "data")]
    pub project_id: String,

    #[serde(rename = "reserved1")]
    pub app_id: String,

    #[serde(rename = "reserved2")]
    pub api_key: String,

    #[serde(rename = "reserved3")]
    pub storage_bucket: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AdMob {
    pub ad_units: Vec<AdUnit>,
    pub test_devices: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AdUnit {
    pub id: String,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct GoogleMap {
    #[serde(rename = "data")]
    pub api_key: String,
}