extern crate swrs;

use swrs::parser::library::{Library, LibraryItem};
use swrs::parser::Parsable;

#[test]
fn parse_library_item() {
    let input = r#"{"adUnits":[],"data":"","libType":0,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"N"}"#;
    let result = match LibraryItem::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse library item: {:?}", err),
    };

    let expected = LibraryItem {
        ad_units: vec![],
        lib_type: 0,
        data: "".to_string(),
        reserved1: "".to_string(),
        reserved2: "".to_string(),
        reserved3: "".to_string(),
        test_devices: vec![],
        use_yn: "N".to_string(),
    };

    assert_eq!(result, expected);
}

#[test]
fn parse_library() {
    let input = r#"@firebaseDB
{"adUnits":[],"data":"","libType":0,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"N"}
@compat
{"adUnits":[],"data":"","libType":1,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"Y"}
@admob
{"adUnits":[],"data":"","libType":2,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"N"}
@googleMap
{"adUnits":[],"data":"","libType":3,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"N"}"#;

    let result = match Library::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse library: {:?}", err),
    };

    let expected = Library {
        firebase_db: LibraryItem {
            ad_units: vec![],
            lib_type: 0,
            data: "".to_string(),
            reserved1: "".to_string(),
            reserved2: "".to_string(),
            reserved3: "".to_string(),
            test_devices: vec![],
            use_yn: "N".to_string(),
        },
        compat: LibraryItem {
            ad_units: vec![],
            lib_type: 1,
            data: "".to_string(),
            reserved1: "".to_string(),
            reserved2: "".to_string(),
            reserved3: "".to_string(),
            test_devices: vec![],
            use_yn: "Y".to_string(),
        },
        admob: LibraryItem {
            ad_units: vec![],
            lib_type: 2,
            data: "".to_string(),
            reserved1: "".to_string(),
            reserved2: "".to_string(),
            reserved3: "".to_string(),
            test_devices: vec![],
            use_yn: "N".to_string(),
        },
        google_map: LibraryItem {
            ad_units: vec![],
            lib_type: 3,
            data: "".to_string(),
            reserved1: "".to_string(),
            reserved2: "".to_string(),
            reserved3: "".to_string(),
            test_devices: vec![],
            use_yn: "N".to_string(),
        },
    };

    assert_eq!(result, expected);
}

#[test]
fn reconstruct_library_item() {
    let input = LibraryItem {
        ad_units: vec![],
        lib_type: 0,
        data: "".to_string(),
        reserved1: "".to_string(),
        reserved2: "".to_string(),
        reserved3: "".to_string(),
        test_devices: vec![],
        use_yn: "N".to_string(),
    };

    let result = match input.reconstruct() {
        Ok(r) => r,
        Err(err) => panic!("Failed to reconstruct library item: {:?}", err),
    };

    let expected = r#"{"adUnits":[],"data":"","libType":0,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"N"}"#;

    assert_eq!(result, expected);
}

#[test]
fn reconstruct_library() {
    let input = Library {
        firebase_db: LibraryItem {
            ad_units: vec![],
            lib_type: 0,
            data: "".to_string(),
            reserved1: "".to_string(),
            reserved2: "".to_string(),
            reserved3: "".to_string(),
            test_devices: vec![],
            use_yn: "N".to_string(),
        },
        compat: LibraryItem {
            ad_units: vec![],
            lib_type: 1,
            data: "".to_string(),
            reserved1: "".to_string(),
            reserved2: "".to_string(),
            reserved3: "".to_string(),
            test_devices: vec![],
            use_yn: "Y".to_string(),
        },
        admob: LibraryItem {
            ad_units: vec![],
            lib_type: 2,
            data: "".to_string(),
            reserved1: "".to_string(),
            reserved2: "".to_string(),
            reserved3: "".to_string(),
            test_devices: vec![],
            use_yn: "N".to_string(),
        },
        google_map: LibraryItem {
            ad_units: vec![],
            lib_type: 3,
            data: "".to_string(),
            reserved1: "".to_string(),
            reserved2: "".to_string(),
            reserved3: "".to_string(),
            test_devices: vec![],
            use_yn: "N".to_string(),
        },
    };

    let result = match input.reconstruct() {
        Ok(r) => r,
        Err(err) => panic!("Failed to reconstruct library: {:?}", err),
    };

    let expected = r#"@firebaseDB
{"adUnits":[],"data":"","libType":0,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"N"}
@compat
{"adUnits":[],"data":"","libType":1,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"Y"}
@admob
{"adUnits":[],"data":"","libType":2,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"N"}
@googleMap
{"adUnits":[],"data":"","libType":3,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"N"}"#;

    assert_eq!(result, expected);
}
