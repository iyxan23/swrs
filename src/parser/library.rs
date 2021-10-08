use json::JsonValue;
use crate::error::{ParseError, SWRSError, SWRSResult};

#[derive(Debug, PartialEq, Eq)]
pub struct Library {
    pub firebase_db: LibraryInfo,
    pub compat: LibraryInfo,
    pub admob: LibraryInfo,
    pub google_map: LibraryInfo,
}

impl Library {
    pub fn parse<S: AsRef<str>>(library: S) -> SWRSResult<Library> {
        let library = library.as_ref();

        let mut firebase_db: Option<LibraryInfo> = Option::None;
        let mut compat: Option<LibraryInfo> = Option::None;
        let mut admob: Option<LibraryInfo> = Option::None;
        let mut google_map: Option<LibraryInfo> = Option::None;

        let mut iterator = library.split("\n");

        loop {
            let line = iterator.next();
            if line.is_none() { break }
            let line = line.unwrap();

            fn parse_library_info(line: &str) -> SWRSResult<LibraryInfo> {
                let parsed = json::parse(line);
                if let Err(e) = &parsed {
                    return Err(
                        SWRSError::ParseError(
                            ParseError {
                                title: format!("Failed parsing a line in library"),
                                description: format!("Line: \n\t{}\nErr: {}", line, e)
                            }
                        )
                    );
                }

                Ok(LibraryInfo::from(parsed.unwrap()))
            }

            match line {
                "@firebaseDB" => firebase_db = Option::Some(parse_library_info(iterator.next().unwrap())?),
                "@compat" => compat = Option::Some(parse_library_info(iterator.next().unwrap())?),
                "@admob" => admob = Option::Some(parse_library_info(iterator.next().unwrap())?),
                "@googleMap" => google_map = Option::Some(parse_library_info(iterator.next().unwrap())?),

                &_ => {}
            }
        }

        Ok(Library {
            firebase_db: firebase_db.unwrap(),
            compat: compat.unwrap(),
            admob: admob.unwrap(),
            google_map: google_map.unwrap()
        })
    }
}

/// Info about a library
///
/// JSON Form:
/// ```json
/// {"adUnits":[],"data":"","libType":0,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"N"}
/// ```
///
/// Note: data, and reserved fields are going to be set None if it is empty
#[derive(Debug, PartialEq, Eq)]
pub struct LibraryInfo {
    pub is_used: bool,
    pub lib_type: u8,
    pub data: Option<String>,
    pub reserved1: Option<String>,
    pub reserved2: Option<String>,
    pub reserved3: Option<String>,

    // test_devices and ad_units' values are currently unknown, will update these when I found an example of them
    pub ad_units: Option<Vec<String>>,
    pub test_devices: Option<Vec<String>>,
}

impl From<JsonValue> for LibraryInfo {
    fn from(json: JsonValue) -> Self {
        LibraryInfo {
            is_used: json["useYn"].as_str().unwrap() == "Y", // if "useYn" is Y, then it is used, otherwise no
            lib_type: json["libType"].as_u8().unwrap(),
            data: get_string_none_if_empty(&json, "data"),
            reserved1: get_string_none_if_empty(&json, "reserved1"),
            reserved2: get_string_none_if_empty(&json, "reserved2"),
            reserved3: get_string_none_if_empty(&json, "reserved3"),
            test_devices: None,
            ad_units: None,
        }
    }
}

fn get_string_none_if_empty(json_val: &JsonValue, key: &str) -> Option<String> {
    let data = json_val[key].as_str().unwrap();

    if data == "" {
        None
    } else {
        Some(data.to_string())
    }
}

impl Library {

}