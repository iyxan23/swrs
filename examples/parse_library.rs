extern crate swrs;

use swrs::parser::library::{Library, LibraryItem};
use swrs::parser::Parsable;

fn main() {
    let library = r#"@firebaseDB
{"adUnits":[],"data":"project_id","libType":0,"reserved1":"app_id","reserved2":"api_key","reserved3":"storage_bucket","testDevices":[],"useYn":"Y"}
@compat
{"adUnits":[],"data":"","libType":1,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"Y"}
@admob
{"adUnits":[],"data":"","libType":2,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"N"}
@googleMap
{"adUnits":[],"data":"google map api key","libType":3,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"Y"}"#;

    let expected = Library {
        firebase_db: LibraryItem {
            ad_units: vec![],
            data: "project_id".to_string(),
            lib_type: 0,
            reserved1: "app_id".to_string(),
            reserved2: "api_key".to_string(),
            reserved3: "storage_bucket".to_string(),
            test_devices: vec![],
            use_yn: "Y".to_string()
        },
        compat: LibraryItem {
            ad_units: vec![],
            data: "".to_string(),
            lib_type: 1,
            reserved1: "".to_string(),
            reserved2: "".to_string(),
            reserved3: "".to_string(),
            test_devices: vec![],
            use_yn: "Y".to_string()
        },
        admob: LibraryItem {
            ad_units: vec![],
            data: "".to_string(),
            lib_type: 2,
            reserved1: "".to_string(),
            reserved2: "".to_string(),
            reserved3: "".to_string(),
            test_devices: vec![],
            use_yn: "N".to_string()
        },
        google_map: LibraryItem {
            ad_units: vec![],
            data: "google map api key".to_string(),
            lib_type: 3,
            reserved1: "".to_string(),
            reserved2: "".to_string(),
            reserved3: "".to_string(),
            test_devices: vec![],
            use_yn: "Y".to_string()
        }
    };

    let parsed_library = Library::parse(library).unwrap();

    assert_eq!(parsed_library, expected);
    println!("{:?}", parsed_library);
}
