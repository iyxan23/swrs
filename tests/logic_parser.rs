use std::collections::HashMap;
use swrs::parser::logic::variable::{Variable, VariablePool, VariableType};
use swrs::parser::Parsable;

#[test]
fn parse_variable_0() {
    let input = "1:my_int";

    let result = match Variable::parse(input) {
        Ok(val) => val,
        Err(err) => panic!("Failed to parse variable: {}", err)
    };

    assert_eq!(
        Variable {
            name: "my_int".to_string(),
            r#type: VariableType::Integer
        },
        result
    );
}

#[test]
fn parse_variable_1() {
    let input = "3:variable_with_a_very_long_name";

    let result = match Variable::parse(input) {
        Ok(val) => val,
        Err(err) => panic!("Failed to parse variable: {}", err)
    };

    assert_eq!(
        Variable {
            name: "variable_with_a_very_long_name".to_string(),
            r#type: VariableType::HashMap
        },
        result
    );
}

#[test]
fn parse_variable_pool_0() {
    let input = "\
1:my_int
2:my_str
1:my_2nd_int
0:my_cool_bool
2:my_2nd_str
3:my_hashmap";

    let result = match VariablePool::parse(input) {
        Ok(val) => val,
        Err(err) => panic!("Failed to parse variables: {}", err)
    };

    let mut expected_pool = HashMap::<String, Variable>::new();
    expected_pool.insert("my_int".to_string(), Variable { name: "my_int".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("my_str".to_string(), Variable { name: "my_str".to_string(), r#type: VariableType::String });
    expected_pool.insert("my_2nd_int".to_string(), Variable { name: "my_2nd_int".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("my_cool_bool".to_string(), Variable { name: "my_cool_bool".to_string(), r#type: VariableType::Boolean });
    expected_pool.insert("my_2nd_str".to_string(), Variable { name: "my_2nd_str".to_string(), r#type: VariableType::String });
    expected_pool.insert("my_hashmap".to_string(), Variable { name: "my_hashmap".to_string(), r#type: VariableType::HashMap });

    // we can't do assert_eq since hashmap order items randomly
    for variable in expected_pool {
        if let Some(val) = result.0.get(&*variable.0) {
            assert_eq!(*val, variable.1, "In-equal value of variable {}. Result dump: {:?}", variable.0, result)
        } else {
            panic!("Cannot find field {} in the parsed value", variable.0);
        }
    }
}

#[test]
fn parse_variable_pool_1() {
    let input = "\
1:size
1:size2
2:col
2:blacks
2:reds
2:yellows
2:pointer
1:size3
0:v
0:i
2:pinks
2:purple
2:deepPurple
2:Indigo
2:Blue
2:LightBlue
2:cyan
2:teal
2:green
2:lightGreen
2:lime
2:amber
2:Orange
2:deeoOrange
2:brown
2:gray
2:blueGray
2:transparent
2:canvastitle
2:canvasok
2:cwidth
2:cheight
2:canvastoast
1:canvaswidth
1:canvasheight
1:widthpx
1:heightpx
2:canvascancel
1:tv
0:bl_on
0:bl_off
1:h
1:w
1:canvash
1:canvasw
1:hout
1:wout
2:canvas_path
0:lx
0:ly
1:svd
0:vtmundo
2:image_name
2:folder_path
2:output_path";

    let result = match VariablePool::parse(input) {
        Ok(val) => val,
        Err(err) => panic!("Failed to parse variables: {}", err)
    };

    let mut expected_pool = HashMap::<String, Variable>::new();
    expected_pool.insert("size".to_string(), Variable { name: "size".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("size2".to_string(), Variable { name: "size2".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("col".to_string(), Variable { name: "col".to_string(), r#type: VariableType::String });
    expected_pool.insert("blacks".to_string(), Variable { name: "blacks".to_string(), r#type: VariableType::String });
    expected_pool.insert("reds".to_string(), Variable { name: "reds".to_string(), r#type: VariableType::String });
    expected_pool.insert("yellows".to_string(), Variable { name: "yellows".to_string(), r#type: VariableType::String });
    expected_pool.insert("pointer".to_string(), Variable { name: "pointer".to_string(), r#type: VariableType::String });
    expected_pool.insert("size3".to_string(), Variable { name: "size3".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("v".to_string(), Variable { name: "v".to_string(), r#type: VariableType::Boolean });
    expected_pool.insert("i".to_string(), Variable { name: "i".to_string(), r#type: VariableType::Boolean });
    expected_pool.insert("pinks".to_string(), Variable { name: "pinks".to_string(), r#type: VariableType::String });
    expected_pool.insert("purple".to_string(), Variable { name: "purple".to_string(), r#type: VariableType::String });
    expected_pool.insert("deepPurple".to_string(), Variable { name: "deepPurple".to_string(), r#type: VariableType::String });
    expected_pool.insert("Indigo".to_string(), Variable { name: "Indigo".to_string(), r#type: VariableType::String });
    expected_pool.insert("Blue".to_string(), Variable { name: "Blue".to_string(), r#type: VariableType::String });
    expected_pool.insert("LightBlue".to_string(), Variable { name: "LightBlue".to_string(), r#type: VariableType::String });
    expected_pool.insert("cyan".to_string(), Variable { name: "cyan".to_string(), r#type: VariableType::String });
    expected_pool.insert("teal".to_string(), Variable { name: "teal".to_string(), r#type: VariableType::String });
    expected_pool.insert("green".to_string(), Variable { name: "green".to_string(), r#type: VariableType::String });
    expected_pool.insert("lightGreen".to_string(), Variable { name: "lightGreen".to_string(), r#type: VariableType::String });
    expected_pool.insert("lime".to_string(), Variable { name: "lime".to_string(), r#type: VariableType::String });
    expected_pool.insert("amber".to_string(), Variable { name: "amber".to_string(), r#type: VariableType::String });
    expected_pool.insert("Orange".to_string(), Variable { name: "Orange".to_string(), r#type: VariableType::String });
    expected_pool.insert("deeoOrange".to_string(), Variable { name: "deeoOrange".to_string(), r#type: VariableType::String });
    expected_pool.insert("brown".to_string(), Variable { name: "brown".to_string(), r#type: VariableType::String });
    expected_pool.insert("gray".to_string(), Variable { name: "gray".to_string(), r#type: VariableType::String });
    expected_pool.insert("blueGray".to_string(), Variable { name: "blueGray".to_string(), r#type: VariableType::String });
    expected_pool.insert("transparent".to_string(), Variable { name: "transparent".to_string(), r#type: VariableType::String });
    expected_pool.insert("canvastitle".to_string(), Variable { name: "canvastitle".to_string(), r#type: VariableType::String });
    expected_pool.insert("canvasok".to_string(), Variable { name: "canvasok".to_string(), r#type: VariableType::String });
    expected_pool.insert("cwidth".to_string(), Variable { name: "cwidth".to_string(), r#type: VariableType::String });
    expected_pool.insert("cheight".to_string(), Variable { name: "cheight".to_string(), r#type: VariableType::String });
    expected_pool.insert("canvastoast".to_string(), Variable { name: "canvastoast".to_string(), r#type: VariableType::String });
    expected_pool.insert("canvaswidth".to_string(), Variable { name: "canvaswidth".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("canvasheight".to_string(), Variable { name: "canvasheight".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("widthpx".to_string(), Variable { name: "widthpx".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("heightpx".to_string(), Variable { name: "heightpx".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("canvascancel".to_string(), Variable { name: "canvascancel".to_string(), r#type: VariableType::String });
    expected_pool.insert("tv".to_string(), Variable { name: "tv".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("bl_on".to_string(), Variable { name: "bl_on".to_string(), r#type: VariableType::Boolean });
    expected_pool.insert("bl_off".to_string(), Variable { name: "bl_off".to_string(), r#type: VariableType::Boolean });
    expected_pool.insert("h".to_string(), Variable { name: "h".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("w".to_string(), Variable { name: "w".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("canvash".to_string(), Variable { name: "canvash".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("canvasw".to_string(), Variable { name: "canvasw".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("hout".to_string(), Variable { name: "hout".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("wout".to_string(), Variable { name: "wout".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("canvas_path".to_string(), Variable { name: "canvas_path".to_string(), r#type: VariableType::String });
    expected_pool.insert("lx".to_string(), Variable { name: "lx".to_string(), r#type: VariableType::Boolean });
    expected_pool.insert("ly".to_string(), Variable { name: "ly".to_string(), r#type: VariableType::Boolean });
    expected_pool.insert("svd".to_string(), Variable { name: "svd".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("vtmundo".to_string(), Variable { name: "vtmundo".to_string(), r#type: VariableType::Boolean });
    expected_pool.insert("image_name".to_string(), Variable { name: "image_name".to_string(), r#type: VariableType::String });
    expected_pool.insert("folder_path".to_string(), Variable { name: "folder_path".to_string(), r#type: VariableType::String });
    expected_pool.insert("output_path".to_string(), Variable { name: "output_path".to_string(), r#type: VariableType::String });

    // we can't do assert_eq since hashmap order items randomly
    for variable in expected_pool {
        if let Some(val) = result.0.get(&*variable.0) {
            assert_eq!(*val, variable.1, "In-equal value of variable {}. Result dump: {:?}", variable.0, result)
        } else {
            panic!("Cannot find field {} in the parsed value", variable.0);
        }
    }
}