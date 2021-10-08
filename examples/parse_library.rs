extern crate swrs;

use swrs::parser::library::Library;

fn main() {
    let library = r#"@firebaseDB
{"adUnits":[],"data":"","libType":0,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"N"}
@compat
{"adUnits":[],"data":"","libType":1,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"N"}
@admob
{"adUnits":[],"data":"","libType":2,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"N"}
@googleMap
{"adUnits":[],"data":"","libType":3,"reserved1":"","reserved2":"","reserved3":"","testDevices":[],"useYn":"N"}"#;

    println!("{:?}", Library::parse(library).unwrap());
}