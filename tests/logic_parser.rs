use std::collections::HashMap;
use ritelinked::LinkedHashMap;
use swrs::parser::logic::Logic;
use swrs::parser::logic::component::{Component};
use swrs::parser::logic::event::{Event, EventPool};
use swrs::parser::logic::list_variable::{ListVariable, ListVariablePool};
use swrs::parser::logic::more_block::{MoreBlock, MoreBlockPool};
use swrs::parser::logic::variable::{Variable, VariablePool, VariableType};
use swrs::parser::Parsable;

// Variables =======================================================================================

#[test]
fn parse_variable_0() {
    let input = "1:my_int";

    let result = match Variable::parse(input) {
        Ok(val) => val,
        Err(err) => panic!("Failed to parse variable: {:?}", err)
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
        Err(err) => panic!("Failed to parse variable: {:?}", err)
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

    let result = match <VariablePool as Parsable>::parse(input) {
        Ok(val) => val,
        Err(err) => panic!("Failed to parse variables: {:?}", err)
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

    let result = match <VariablePool as Parsable>::parse(input) {
        Ok(val) => val,
        Err(err) => panic!("Failed to parse variables: {:?}", err)
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

// Variables =======================================================================================

// List variables ==================================================================================

#[test]
fn parse_list_variable() {
    let input = "0:my_booleans";
    let result = match ListVariable::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse list variable: {:?}", err)
    };

    assert_eq!(
        result,
        ListVariable {
            name: "my_booleans".to_string(),
            r#type: VariableType::Boolean
        }
    )
}

#[test]
fn parse_list_variable_pool() {
    let input = r#"0:booleans
1:integers
2:strings
3:maps"#;
    let result = match ListVariablePool::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse list variable pool: {:?}", err)
    };

    let expected = {
        let mut map = LinkedHashMap::<String, ListVariable>::new();

        map.insert("booleans".to_string(), ListVariable { name: "booleans".to_string(), r#type: VariableType::Boolean });
        map.insert("integers".to_string(), ListVariable { name: "integers".to_string(), r#type: VariableType::Integer });
        map.insert("strings".to_string(), ListVariable { name: "strings".to_string(), r#type: VariableType::String });
        map.insert("maps".to_string(), ListVariable { name: "maps".to_string(), r#type: VariableType::HashMap });

        ListVariablePool(map)
    };

    assert_eq!(expected, result)
}

// List variables ==================================================================================

// Components ======================================================================================

#[test]
fn parse_component() {
    let input = r#"{"componentId":"dialog","param1":"","param2":"","param3":"","type":7}"#;
    let result = match Component::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse component: {:?}", err)
    };

    let expected = Component {
        id: "dialog".to_string(),
        param1: "".to_string(),
        param2: "".to_string(),
        param3: "".to_string(),
        r#type: 7
    };

    assert_eq!(result, expected);
}

// Components ======================================================================================

// Events ==========================================================================================

#[test]
fn parse_event() {
    let input = r#"{"eventName":"onClick","eventType":1,"targetId":"linear10","targetType":0}"#;
    let result = match Event::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse event: {:?}", err)
    };

    let expected = Event {
        event_name: "onClick".to_string(),
        event_type: 1,
        target_id: "linear10".to_string(),
        target_type: 0
    };

    assert_eq!(result, expected);
}

#[test]
fn parse_event_pool() {
    let input = r#"{"eventName":"onResponse","eventType":2,"targetId":"ping_test","targetType":17}
{"eventName":"onErrorResponse","eventType":2,"targetId":"ping_test","targetType":17}
{"eventName":"onClick","eventType":1,"targetId":"item_2","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"item_3","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"item_5","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"item_1","targetType":0}
{"eventName":"onCheckedChange","eventType":1,"targetId":"switch_theme","targetType":13}
{"eventName":"onResponse","eventType":2,"targetId":"get_ip","targetType":17}
{"eventName":"onClick","eventType":1,"targetId":"aver","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"dsdk","targetType":0}"#;

    let result = match EventPool::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse event pool: {:?}", err)
    };

    let expected = EventPool(vec![
        Event { event_name: "onResponse".to_string(), event_type: 2, target_id: "ping_test".to_string(), target_type: 17 },
        Event { event_name: "onErrorResponse".to_string(), event_type: 2, target_id: "ping_test".to_string(), target_type: 17 },
        Event { event_name: "onClick".to_string(), event_type: 1, target_id: "item_2".to_string(), target_type: 0 },
        Event { event_name: "onClick".to_string(), event_type: 1, target_id: "item_3".to_string(), target_type: 0 },
        Event { event_name: "onClick".to_string(), event_type: 1, target_id: "item_5".to_string(), target_type: 0 },
        Event { event_name: "onClick".to_string(), event_type: 1, target_id: "item_1".to_string(), target_type: 0 },
        Event { event_name: "onCheckedChange".to_string(), event_type: 1, target_id: "switch_theme".to_string(), target_type: 13 },
        Event { event_name: "onResponse".to_string(), event_type: 2, target_id: "get_ip".to_string(), target_type: 17 },
        Event { event_name: "onClick".to_string(), event_type: 1, target_id: "aver".to_string(), target_type: 0 },
        Event { event_name: "onClick".to_string(), event_type: 1, target_id: "dsdk".to_string(), target_type: 0 },
    ]);

    assert_eq!(result, expected);
}

// Events ==========================================================================================

// Functions / MoreBlocks ==========================================================================

#[test]
fn parse_more_block() {
    let input = "cool_moreblock:hello world %s";
    let result = match MoreBlock::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse moreblock: {:?}", err)
    };

    let expected = MoreBlock {
        id: "cool_moreblock".to_string(),
        spec: "hello world %s".to_string()
    };

    assert_eq!(result, expected);
}

#[test]
fn parse_more_block_pool_0() {
    let input = "cool_moreblock:hello world %s\nhello_world:very poggers %s cool %i";
    let result = match MoreBlockPool::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse moreblock pool: {:?}", err)
    };

    let mut expected = LinkedHashMap::<String, MoreBlock>::new();
    expected.insert("cool_moreblock".to_string(), MoreBlock { id: "cool_moreblock".to_string(), spec: "hello world %s".to_string() });
    expected.insert("hello_world".to_string(), MoreBlock { id: "hello_world".to_string(), spec: "very poggers %s cool %i".to_string() });

    assert_eq!(result, MoreBlockPool(expected));
}

#[test]
fn parse_more_block_pool_1() {
    let input = "\
CardView:CardView %s.color %d.radius %d.shadow %m.view.view
setBackgroundLinear:setBackgroundLinear %m.view.view fromFilePath %s.path
SaveLinear:SaveLinear %m.view.view to path %s.path
edtDial:edtDial Nazv %s.str1 Mess %s.str2 Hint %s.hint Dialog %m.dialog.dia Output %m.textview.getMess
save_and_exit:save_and_exit Nazv %s.str1 Mess %s.str2 Hint %s.hint Dialog %m.dialog.dia Output %m.textview.getMess
canvas:canvas %d.width %d.height
setCornerRadius:setCornerRadius to %m.view.view percentage %d.percent
EditTextLimit:EditTextLimit %m.textview.edittext setLimit %d.limit output %m.textview.output_tv
saveView:saveView %m.view.view folderPath %s.folderPath outputPath %s.outputPath";

    let result = match MoreBlockPool::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse moreblock pool: {:?}", err)
    };

    let mut expected = LinkedHashMap::<String, MoreBlock>::new();
    expected.insert("CardView".to_string(), MoreBlock { id: "CardView".to_string(), spec: "CardView %s.color %d.radius %d.shadow %m.view.view".to_string() });
    expected.insert("setBackgroundLinear".to_string(), MoreBlock { id: "setBackgroundLinear".to_string(), spec: "setBackgroundLinear %m.view.view fromFilePath %s.path".to_string() });
    expected.insert("SaveLinear".to_string(), MoreBlock { id: "SaveLinear".to_string(), spec: "SaveLinear %m.view.view to path %s.path".to_string() });
    expected.insert("edtDial".to_string(), MoreBlock { id: "edtDial".to_string(), spec: "edtDial Nazv %s.str1 Mess %s.str2 Hint %s.hint Dialog %m.dialog.dia Output %m.textview.getMess".to_string() });
    expected.insert("save_and_exit".to_string(), MoreBlock { id: "save_and_exit".to_string(), spec: "save_and_exit Nazv %s.str1 Mess %s.str2 Hint %s.hint Dialog %m.dialog.dia Output %m.textview.getMess".to_string() });
    expected.insert("canvas".to_string(), MoreBlock { id: "canvas".to_string(), spec: "canvas %d.width %d.height".to_string() });
    expected.insert("setCornerRadius".to_string(), MoreBlock { id: "setCornerRadius".to_string(), spec: "setCornerRadius to %m.view.view percentage %d.percent".to_string() });
    expected.insert("EditTextLimit".to_string(), MoreBlock { id: "EditTextLimit".to_string(), spec: "EditTextLimit %m.textview.edittext setLimit %d.limit output %m.textview.output_tv".to_string() });
    expected.insert("saveView".to_string(), MoreBlock { id: "saveView".to_string(), spec: "saveView %m.view.view folderPath %s.folderPath outputPath %s.outputPath".to_string() });

    assert_eq!(result, MoreBlockPool(expected));
}

// Functions / MoreBlocks ==========================================================================

// Full logic ======================================================================================

#[test]
fn parse_logic_0() {
    // A project of mine
    let input = r#"@MainActivity.java_var
2:out

@MainActivity.java_func
execute_shell:execute_shell %s.command

@MainActivity.java_onCreate_initializeLogic
{"color":-10701022,"id":"12","nextBlock":10,"opCode":"addSourceDirectly","parameters":["// For those who are familiar with Linux commands, we\u0027re reading /proc/cpuinfo, and /proc/meminfo"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"10","nextBlock":13,"opCode":"definedFunc","parameters":["cat /proc/cpuinfo"],"spec":"execute_shell %s.command","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":11,"opCode":"setText","parameters":["cpuinfo","@16"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"16","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"out","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-7711273,"id":"11","nextBlock":14,"opCode":"definedFunc","parameters":["cat /proc/meminfo"],"spec":"execute_shell %s.command","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":-1,"opCode":"setText","parameters":["raminfo","@15"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"15","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"out","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}

@MainActivity.java_execute_shell_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["StringBuilder output \u003d new StringBuilder();\ntry {\njava.lang.Process cmdProc \u003d Runtime.getRuntime().exec(_command);\n\n\njava.io.BufferedReader stdoutReader \u003d new java.io.BufferedReader(\n         new java.io.InputStreamReader(cmdProc.getInputStream()));\nString line;\nwhile ((line \u003d stdoutReader.readLine()) !\u003d null) {\n   // process procs standard output here\n  output.append(line + \"\\n\");\n}\n\nthis.out \u003d output.toString();\n\n} catch (java.io.IOException e) {\nthis.out \u003d \"Error occurred\";\n}"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}"#;

    match Logic::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse logic: {:?}", err)
    };

    // sadly i can't check if it produces a correct output, if someone got time, please do this
}

#[test]
fn reconstruct_logic() {
    // A project of mine
    let input = r#"@MainActivity.java_var
2:out

@MainActivity.java_func
execute_shell:execute_shell %s.command

@MainActivity.java_onCreate_initializeLogic
{"color":-10701022,"id":"12","nextBlock":10,"opCode":"addSourceDirectly","parameters":["// For those who are familiar with Linux commands, we're reading /proc/cpuinfo, and /proc/meminfo"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"10","nextBlock":13,"opCode":"definedFunc","parameters":["cat /proc/cpuinfo"],"spec":"execute_shell %s.command","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":11,"opCode":"setText","parameters":["cpuinfo","@16"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"16","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"out","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-7711273,"id":"11","nextBlock":14,"opCode":"definedFunc","parameters":["cat /proc/meminfo"],"spec":"execute_shell %s.command","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":-1,"opCode":"setText","parameters":["raminfo","@15"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"15","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"out","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}

@MainActivity.java_execute_shell_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["StringBuilder output = new StringBuilder();\ntry {\njava.lang.Process cmdProc = Runtime.getRuntime().exec(_command);\n\n\njava.io.BufferedReader stdoutReader = new java.io.BufferedReader(\n         new java.io.InputStreamReader(cmdProc.getInputStream()));\nString line;\nwhile ((line = stdoutReader.readLine()) != null) {\n   // process procs standard output here\n  output.append(line + \"\\n\");\n}\n\nthis.out = output.toString();\n\n} catch (java.io.IOException e) {\nthis.out = \"Error occurred\";\n}"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}"#;

    let result = match Logic::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse logic: {:?}", err)
    };

    let reconstructed = match result.reconstruct() {
        Ok(r) => r,
        Err(err) => panic!("Failed to reconstruct logic: {:?}", err)
    };

    assert_eq!(input, reconstructed);
}