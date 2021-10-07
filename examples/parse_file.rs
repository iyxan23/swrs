extern crate swrs;

use swrs::parser::file::File;

fn main() {
    let file = r#"@activity
{"fileName":"main","fileType":0,"keyboardSetting":0,"options":1,"orientation":0,"theme":-1}
@customview"#;

    println!("{:?}", File::parse(file).unwrap());
}