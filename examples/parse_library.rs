extern crate swrs;

use swrs::parser::library::{FirebaseDB, GoogleMap, Library};
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
        firebase_db: Some(FirebaseDB {
            project_id: "project_id".to_string(),
            app_id: "app_id".to_string(),
            api_key: "api_key".to_string(),
            storage_bucket: "storage_bucket".to_string(),
        }),
        admob: None,
        google_map: Some(GoogleMap {
            api_key: "google map api key".to_string(),
        }),
        appcompat_enabled: true
    };

    let parsed_library = Library::parse(library).unwrap();

    assert_eq!(parsed_library, expected);
    println!("{:?}", parsed_library);
}
